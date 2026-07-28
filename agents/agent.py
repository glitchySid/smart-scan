import argparse

from langchain_core.prompts import ChatPromptTemplate
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables import RunnablePassthrough
from langchain_ollama import ChatOllama
from sentence_transformers import SentenceTransformer

from config import LLM_MODEL, EMBEDDING_MODEL, get_vectorstore

RAG_PROMPT = ChatPromptTemplate.from_template(
    """You are a helpful assistant that answers questions about screenshots.
Use the provided context to answer the question. If the context doesn't contain
enough information, say so honestly.

Context:
{context}

Question: {question}

Answer:"""
)


def format_docs(docs):
    formatted = []
    for i, doc in enumerate(docs, 1):
        file_info = doc.get("file", "unknown")
        formatted.append(f"[{i}] (File: {file_info})\n{doc.get('content', '')}")
    return "\n\n".join(formatted)


def rag_query(question: str, k: int = 5) -> dict:
    model = SentenceTransformer(EMBEDDING_MODEL)
    query_embedding = model.encode([question])[0].tolist()

    vectorstore = get_vectorstore()
    results = vectorstore.search(query_embedding).limit(k).to_list()

    docs = [
        {"file": r.get("file", ""), "content": r.get("content", "")}
        for r in results
    ]

    llm = ChatOllama(model=LLM_MODEL, temperature=0.1)

    chain = (
        {"context": lambda x: format_docs(docs), "question": RunnablePassthrough()}
        | RAG_PROMPT
        | llm
        | StrOutputParser()
    )

    answer = chain.invoke(question)

    sources = [
        {"content": doc["content"][:200], "file": doc["file"]}
        for doc in docs
    ]

    return {
        "status": "ok",
        "question": question,
        "answer": answer,
        "sources": sources,
    }


def main():
    parser = argparse.ArgumentParser(description="RAG agent for screenshot Q&A")
    parser.add_argument("--query", required=True, help="Question to answer")
    parser.add_argument("--k", type=int, default=5, help="Number of context chunks")
    args = parser.parse_args()

    result = rag_query(args.query, args.k)

    print(f"\n{result['answer']}\n")

    if result["sources"]:
        print("Sources:")
        for src in result["sources"]:
            fname = src["file"].rsplit("/", 1)[-1]
            print(f"  - {fname}")


if __name__ == "__main__":
    main()

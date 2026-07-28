import argparse
import json

from sentence_transformers import SentenceTransformer

from config import EMBEDDING_MODEL, get_vectorstore


def semantic_search(query: str, k: int = 5) -> dict:
    model = SentenceTransformer(EMBEDDING_MODEL)
    query_embedding = model.encode([query])[0].tolist()

    vectorstore = get_vectorstore()
    results = vectorstore.search(query_embedding).limit(k).to_list()

    formatted = [
        {
            "content": r.get("content", ""),
            "metadata": {"file": r.get("file", "")},
        }
        for r in results
    ]

    return {
        "status": "ok",
        "query": query,
        "count": len(formatted),
        "results": formatted,
    }


def main():
    parser = argparse.ArgumentParser(description="Semantic search over screenshots")
    parser.add_argument("--query", required=True, help="Search query")
    parser.add_argument("--k", type=int, default=5, help="Number of results")
    args = parser.parse_args()

    result = semantic_search(args.query, args.k)
    print(json.dumps(result))


if __name__ == "__main__":
    main()

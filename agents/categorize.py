import argparse
import json

from sentence_transformers import SentenceTransformer
import numpy as np

from config import (
    CATEGORIES,
    EMBEDDING_MODEL,
    SIMILARITY_THRESHOLD,
    get_vectorstore,
)


def categorize_screenshot(file_path: str, content: str) -> dict:
    model = SentenceTransformer(EMBEDDING_MODEL)

    category_embeddings = model.encode(CATEGORIES)
    content_embedding = model.encode([content])

    similarities = np.dot(category_embeddings, content_embedding.T).flatten()
    best_idx = int(np.argmax(similarities))
    best_score = float(similarities[best_idx])

    sorted_indices = np.argsort(similarities)[::-1]
    top_categories = [
        {
            "category": CATEGORIES[i],
            "score": float(similarities[i]),
        }
        for i in sorted_indices[:3]
    ]

    assigned = CATEGORIES[best_idx] if best_score >= SIMILARITY_THRESHOLD else "other"

    vectorstore = get_vectorstore()
    vectorstore.add(
        [
            {
                "file": file_path,
                "content": content,
                "vector": content_embedding[0].tolist(),
                "category": assigned,
                "confidence": str(best_score),
            }
        ]
    )

    return {
        "status": "ok",
        "file": file_path,
        "assigned_category": assigned,
        "confidence": best_score,
        "top_categories": top_categories,
    }


def categorize_batch(file_paths: list[str], contents: list[str]) -> dict:
    model = SentenceTransformer(EMBEDDING_MODEL)

    category_embeddings = model.encode(CATEGORIES)
    content_embeddings = model.encode(contents)

    results = []
    vectorstore = get_vectorstore()

    for i, (file_path, content) in enumerate(zip(file_paths, contents)):
        similarities = np.dot(category_embeddings, content_embeddings[i]).flatten()
        best_idx = int(np.argmax(similarities))
        best_score = float(similarities[best_idx])

        sorted_indices = np.argsort(similarities)[::-1]
        top_categories = [
            {
                "category": CATEGORIES[j],
                "score": float(similarities[j]),
            }
            for j in sorted_indices[:3]
        ]

        assigned = CATEGORIES[best_idx] if best_score >= SIMILARITY_THRESHOLD else "other"

        vectorstore.add(
            [
                {
                    "file": file_path,
                    "content": content,
                    "vector": content_embeddings[i].tolist(),
                    "category": assigned,
                    "confidence": str(best_score),
                }
            ]
        )

        results.append(
            {
                "file": file_path,
                "assigned_category": assigned,
                "confidence": best_score,
                "top_categories": top_categories,
            }
        )

    return {
        "status": "ok",
        "count": len(results),
        "results": results,
    }


def main():
    parser = argparse.ArgumentParser(description="Categorize screenshots")
    parser.add_argument("--file", help="Single file path to categorize")
    parser.add_argument("--text", help="OCR text content")
    parser.add_argument("--files", nargs="+", help="Multiple file paths")
    parser.add_argument("--texts", nargs="+", help="Multiple OCR texts")
    args = parser.parse_args()

    if args.file and args.text:
        result = categorize_screenshot(args.file, args.text)
    elif args.files and args.texts:
        if len(args.files) != len(args.texts):
            print(
                json.dumps({"status": "error", "message": "files and texts length mismatch"})
            )
            return
        result = categorize_batch(args.files, args.texts)
    else:
        print(json.dumps({"status": "error", "message": "provide --file/--text or --files/--texts"}))
        return

    print(json.dumps(result))


if __name__ == "__main__":
    main()

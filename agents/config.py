import os
from pathlib import Path

from dotenv import load_dotenv
import lancedb

load_dotenv(Path(__file__).parent / ".env")

PROJECT_ROOT = Path(__file__).parent.parent
LANCE_DIR = Path(os.environ.get("LANCE_DIR", str(PROJECT_ROOT / "lance_db")))
TABLE_NAME = os.environ.get("TABLE_NAME", "screenshots")

EMBEDDING_MODEL = os.environ.get("EMBEDDING_MODEL", "all-MiniLM-L6-v2")
LLM_MODEL = os.environ.get("LLM_MODEL", "llama3.2:1b")
CHUNK_SIZE = int(os.environ.get("CHUNK_SIZE", "500"))
CHUNK_OVERLAP = int(os.environ.get("CHUNK_OVERLAP", "50"))
SIMILARITY_THRESHOLD = float(os.environ.get("SIMILARITY_THRESHOLD", "0.3"))

CATEGORIES = [
    "code",
    "design",
    "chat",
    "docs",
    "error",
    "meeting",
    "research",
    "finance",
    "social",
    "news",
    "shopping",
    "other",
]


def get_vectorstore():
    db = lancedb.connect(str(LANCE_DIR))
    return db.open_table(TABLE_NAME)

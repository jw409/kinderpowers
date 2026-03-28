import httpx
import json
import subprocess
import shutil
import logging

logger = logging.getLogger(__name__)

def vm_generate(prompt: str, port: int = 8765, model: str = "default", max_tokens: int = 4096, timeout: float = 30.0, json_schema: dict = None) -> str | None:
    """Talk to any OpenAI-compatible local server. Returns str or None."""
    body = {"model": model, "messages": [{"role": "user", "content": prompt}], "max_tokens": max_tokens}
    if json_schema:
        body["response_format"] = {"type": "json_schema", "json_schema": json_schema}
    try:
        r = httpx.post(f"http://127.0.0.1:{port}/v1/chat/completions", json=body, timeout=timeout)
        r.raise_for_status()
        return r.json()["choices"][0]["message"]["content"]
    except Exception as e:
        logger.debug(f"vm_generate failed on port {port}: {e}")
        return None

def gemini_generate(prompt: str, max_tokens: int = 4096) -> str | None:
    """Gemini CLI (free OAuth). Returns str or None."""
    if not shutil.which("gemini"):
        return None
    try:
        r = subprocess.run(["gemini", "generate", "--max-tokens", str(max_tokens), prompt],
                           capture_output=True, text=True, timeout=60)
        return r.stdout.strip() if r.returncode == 0 else None
    except Exception as e:
        logger.debug(f"gemini_generate failed: {e}")
        return None

def generate(prompt: str, max_tokens: int = 4096, json_schema: dict = None) -> str:
    """Cheapest-first: gemini CLI ($0) → GPU ($0) → cmax ($$$)."""
    # 1. Gemini CLI (free) - CLI doesn't support json_schema
    if not json_schema:
        result = gemini_generate(prompt, max_tokens)
        if result:
            return result
    
    # 2. Local GPU
    result = vm_generate(prompt, port=8765, max_tokens=max_tokens, json_schema=json_schema)
    if result:
        return result
        
    # 3. cmax (costs money)
    result = vm_generate(prompt, port=8889, model="claude-sonnet-4-20250514", max_tokens=max_tokens, json_schema=json_schema)
    if result:
        return result
        
    raise RuntimeError("All inference backends failed")

def health(port: int = 8765) -> bool:
    """Check if a local VM is alive."""
    try:
        r = httpx.get(f"http://127.0.0.1:{port}/v1/models", timeout=5)
        return r.status_code == 200
    except Exception:
        return False

def discover() -> dict[str, int]:
    """Find all live local inference servers."""
    ports = {"gpu": 8765, "cmax": 8889, "ollama": 11434, "lmstudio": 1234, "vllm": 8000}
    return {name: port for name, port in ports.items() if health(port)}

import asyncio
import howl_network
import json

async def main():
    client = howl_network.Client("Provider")
    await client.connect("ws://127.0.0.1:8000")
    await client.share_data(json.dumps({
        "category": "test2_%",
        "number": "12"
    }))

if __name__ == "__main__":
    asyncio.run(main())

import asyncio

from protocol.proxy_pb2 import CreateSessionRequest, CreateSessionResponse


async def main():
    response = CreateSessionResponse(session_id='123')
    print(f"response: {response}")


if __name__ == '__main__':
    asyncio.run(main())
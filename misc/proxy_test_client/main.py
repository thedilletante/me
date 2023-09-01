import asyncio
import grpc
import ffmpeg
import os
import tempfile
import vlc

from protocol.proxy_pb2 import CreateSessionRequest, CreateLegRequest, SetLocalDescriptionRequest
from protocol.proxy_pb2_grpc import ProxyStub


async def main():
    async with grpc.aio.insecure_channel('127.0.0.1:7777') as channel:
        stub = ProxyStub(channel)

        # session = await stub.CreateSession(CreateSessionRequest())
        # print(f"created session: {session}")
        #
        # leg = await stub.CreateLeg(CreateLegRequest(session_id=session.session_id))
        # print(f"created leg: {leg}")

        current_dir = os.path.dirname(os.path.realpath(__file__))

        with tempfile.NamedTemporaryFile(suffix=".sdp") as sdp_file:
            process = (
                ffmpeg
                .input(
                    f"{current_dir}/media/Big_Buck_Bunny_360_10s_1MB.mp4",
                    re=None,
                    stream_loop=-1,
                )
                .output(
                    "rtp://127.0.0.1:7878",
                    format="rtp",
                    codec="copy",
                    sdp_file=sdp_file.name,
                    loglevel="quiet",
                )
                .run_async()
            )

            # ensure ffmpeg has time to start
            await asyncio.sleep(1)

            sdp = sdp_file.read()
            print(sdp)
            # await stub.SetLocalDescription(
            #     SetLocalDescriptionRequest(
            #         leg_id=leg.leg_id,
            #         sdp=sdp,
            #     )
            # )

            vlc_instance = vlc.Instance()
            player = vlc_instance.media_player_new()
            media = vlc_instance.media_new_path(sdp_file.name)
            player.set_media(media)
            player.play()

            process.wait()


if __name__ == '__main__':
    asyncio.run(main())
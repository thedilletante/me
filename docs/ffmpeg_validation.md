Streaming audio/video and generate SDP file via FFMPEG
```bash
ffmpeg -re -i <video_file> -an -f rtp rtp://127.0.0.1:<audio_port> -vn -f rtp rtp://127.0.0.1:<video_port> -sdp_file output.sdp
```
Generated file is actual answer. Receiver should bind that port and accept data from any endpoint.
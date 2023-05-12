# ME

It's me. Media Engine. Process RTP/RTCP, controlled by SDP.

[[_toc_]]

## Phases

### Proxy
Bandwidth estimation

### Transcoder (transport and codec)
* transport (ice, dtls)
* codec
  * audio (pcmu, pcma, g722, opus)
  * video (vp8, vp9, h264, h265, av1)

### SFU
Selectivity based on RTP extensions
* audio level

### MCU

## Control protocol

Candidates:
* MSML ([RFC 5707](https://tools.ietf.org/html/rfc5707))
* MSCML ([RFC 4722](https://tools.ietf.org/html/rfc4722))
* SIP ([RFC 3261](https://tools.ietf.org/html/rfc3261))
* Custom protocol with custom transport

## Metrics

* Measure latency introduced by components
* Measure quality of the media:
  * audio: MOS, POLQA
  * video: PEVQ

## 3rd party

* [webrtc-rs](https://github.com/webrtc-rs/webrtc)
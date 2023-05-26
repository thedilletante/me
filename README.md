# ME

It's me. Media Engine. Process RTP/RTCP, controlled by SDP.

## Phases

Understand requirements for the project.
What do I want to be the result of the project?
* single binary with runtime configuration for every needs
* single code base to build specific binaries from

### Proxy

* MVP scenario:
  * application started to listen on controlling port
  * accept SDP offer for the first leg
  * opens UDP ports for RTP/RTCP/SCTP
  * generate SDP offer for the second leg
  * waits for the SDP answer for the second leg
  * opens UDP ports for RTP/RTCP/SCTP
  * generate SDP answer for the first leg
  * forward RTP/RTCP/SCTP between legs
* SDP features to understand
  * media types
  * bundle
  * rtcp-mux
  * sctp


### Transcoder (transport, codec)
* transport (ice, dtls)
* codec
  * audio (pcmu, pcma, g722, opus)
  * video (vp8, vp9, h264, h265, av1)

### SFU
* Selectivity based on RTP extensions
  * audio level
* Bandwidth estimation

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
* [tokio](https:://github.com/tokio-rs/tokio)

## Primitives for controlling

* Leg - created and updated by SDP offer/answer model.
* Link/Tunnel/Chain - simple element of the media path that can connect two another elements. Created and updated by control protocol.
* Transformer - element of the media path. Performs modification on incoming stream. Created and updated by control protocol.

Considerations:
* Different types of elements could be connected differently
* Some elements may require feedback from the linked element

# flightsuite-overlay-private

Private overlay onto the public [flightsuite](../flightsuite/) flight
controller. Holds the parts of the system that **must not** ship in the
open-core repo:

- target identification (face / gait / re-id / voice)
- biometric payload events
- targeting-aware autopilot modes (if/when added)
- any model weights or tuning configs trained on private datasets

This repo is private by design and stays private. flightsuite is MIT/Apache
+ EAR99-publishable; the overlay is whatever-it-needs-to-be.

## Architecture

```
┌─────────────────────────────────┐    ┌─────────────────────────────────┐
│  flightsuite (public, EAR99)    │    │  flightsuite-overlay-private    │
│                                 │    │                                 │
│  flight-vision                  │    │  track-id                       │
│  ─ YOLO + ByteTrack             │───▶│  ─ ArcFace, OSNet, GaitGraph2   │
│  ─ activity classifier          │    │  ─ ECAPA-TDNN voice id          │
│  ─ payload bus DetectedObject   │    │  ─ enrolled-subject DB          │
│                                 │    │                                 │
│  flight-app-linux               │    │  biometric-payload              │
│  ─ subscribes to flight-vision  │    │  ─ IdentifiedTrack events       │
│  ─ emits MAVLink                │    │  ─ private MAVLink dialect      │
└─────────────────────────────────┘    └─────────────────────────────────┘
                                                  │
                                                  ▼
                                          subscribers may
                                          take action — out
                                          of scope here
```

The overlay never modifies flightsuite source. It depends on flightsuite
crates as path/git deps (read-only) and adds enrichment layers.

## Crates

| Crate | Purpose |
|---|---|
| `track-id` | Consumes `DetectedObject` events; runs ArcFace / OSNet / GaitGraph2 / ECAPA-TDNN; emits enriched events with `identity` + `confidence`. |
| `biometric-payload` | Custom MAVLink dialect for downlink of identified tracks. Separate from flightsuite's payload dialect. |

## Posture

- License: not whitelisted — this repo's contents are not redistributed.
- No publication. No CI to GitHub Actions. No mirror.
- All model weights stay local on disk; no pushes.
- Test data: only enrolled subjects who have given explicit consent.

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!

# Setup

# Run

```bash
sudo gst-launch-1.0 --gst-plugin-path=/usr/lib/gstoct640usrc oct640usrc serial=23010000 ! videoconvert ! xvimagesink sync=false
```
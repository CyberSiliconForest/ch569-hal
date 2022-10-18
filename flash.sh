#!/bin/sh

cargo objcopy --release --example blink -- -O binary app.bin

../wch-ch56x-isp/wch-ch56x-isp flash -r app.bin


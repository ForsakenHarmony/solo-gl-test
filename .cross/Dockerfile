FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:main

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes \
      libegl1-mesa:armhf libegl1-mesa-dev:armhf \
      libgles2-mesa:armhf libgles2-mesa-dev:armhf
#      libgles1-mesa:armhf libgles1-mesa-dev:armhf

FROM archlinux:base AS iso-builder
WORKDIR /iso
RUN pacman -Syu --noconfirm archiso grub && \
  cp -r /usr/share/archiso/configs/baseline/* .
COPY client/x86_64/iso/ .

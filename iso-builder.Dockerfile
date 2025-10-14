FROM archlinux:base AS iso-builder
WORKDIR /iso
COPY client/x86_64/iso/ ./
RUN pacman -Syu --noconfirm archiso grub

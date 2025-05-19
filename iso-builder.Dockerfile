FROM archlinux:base-20250511.0.348143 AS iso-builder
WORKDIR /iso
COPY client/x86_64/iso/ ./
COPY client/x86_64/iso/pacman.conf /etc/pacman.conf
RUN pacman -Syu --noconfirm archiso grub

set -o errexit
set -o nounset

qemu=$1
base_image=$2
boot_image=$3

################################################################################
# Boot image creation

# Use umask rather than chmod to avoid the race hazard
# that would occur between the creation of the clone
# and the call to chmod.
(
    umask 077
    qemu-img create -f qcow2 -b "$base_image" "$boot_image"
)

################################################################################
# Virtual machine instance

qemu_command=(
    "$qemu"

    # If we do not enable this then QEMU will emulate the architecture
    # instead of using hardware-provided virtualization.
    -enable-kvm

    # Run QEMU headless; we do not want to see its graphical output.
    # Do not even redirect the serial port.
    -display none

    # Mount the disk image on /dev/vda,
    # as expected by the operating system.
    -drive "file=$boot_image,if=virtio"
)

exec "${qemu_command[@]}"

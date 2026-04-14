Hi!
This is my OS project.
There isn't anything to show really at the moment, I'm still trying to get the bootloader to work lol

# Task list
- [x] Isolated bootloader run scripts (without the kernel)
- [ ] Separate the bootloader into specific 32-bit x86 mbr bootloader, plan build methods for multi platform support in the kernel
- [ ] Do some 32-bit kernel stuff
  - [ ] Serial port
  - [ ] x86 primitives crate (got, ivt etc)
  - [ ] mess around with the display
- [ ] 64-bit bootloader that comes from the 32-bit one
- [ ] Bootloader testing (ensure we are in real mode, protected mode, unreal mode etc)

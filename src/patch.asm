0x802ff5d4:
; lis r3, 0x8045 Use 804504A0 as ArenaLow
u32 0x3c608045

0x800063ec:
bl init

0x80006458:
bl game_loop
bl 0x80022e74 ; fapGm_Execute__Fv

0x80313b10:
b read_controller

0x802c8278:
b draw

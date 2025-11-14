bits 16
org 0x7c00

mov si, 0

print:
	mov ah, 0x0e
	mov al, [message + si]
	int 0x10
	add si, 1
	cmp byte [message + si], 0
	jne print

bits 64	
;jmp $ ; kernal entry TODO
jne _start

message:
	db "Booting OpenContriOS", 0

times 510 - ($ - $$) db 0
dw 0xAA55

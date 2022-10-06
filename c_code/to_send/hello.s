
hello:     file format elf32-littleriscv


Disassembly of section .text:

00010094 <exit>:
   10094:	ff010113          	addi	sp,sp,-16
   10098:	00000593          	li	a1,0
   1009c:	00812423          	sw	s0,8(sp)
   100a0:	00112623          	sw	ra,12(sp)
   100a4:	00050413          	mv	s0,a0
   100a8:	298000ef          	jal	ra,10340 <__call_exitprocs>
   100ac:	c281a503          	lw	a0,-984(gp) # 119d8 <_global_impure_ptr>
   100b0:	03c52783          	lw	a5,60(a0)
   100b4:	00078463          	beqz	a5,100bc <exit+0x28>
   100b8:	000780e7          	jalr	a5
   100bc:	00040513          	mv	a0,s0
   100c0:	4a8000ef          	jal	ra,10568 <_exit>

000100c4 <register_fini>:
   100c4:	00000793          	li	a5,0
   100c8:	00078863          	beqz	a5,100d8 <register_fini+0x14>
   100cc:	00010537          	lui	a0,0x10
   100d0:	47450513          	addi	a0,a0,1140 # 10474 <__libc_fini_array>
   100d4:	38c0006f          	j	10460 <atexit>
   100d8:	00008067          	ret

000100dc <_start>:
   100dc:	00002197          	auipc	gp,0x2
   100e0:	cd418193          	addi	gp,gp,-812 # 11db0 <__global_pointer$>
   100e4:	c3418513          	addi	a0,gp,-972 # 119e4 <completed.1>
   100e8:	c5018613          	addi	a2,gp,-944 # 11a00 <__BSS_END__>
   100ec:	40a60633          	sub	a2,a2,a0
   100f0:	00000593          	li	a1,0
   100f4:	170000ef          	jal	ra,10264 <memset>
   100f8:	00000517          	auipc	a0,0x0
   100fc:	36850513          	addi	a0,a0,872 # 10460 <atexit>
   10100:	00050863          	beqz	a0,10110 <_start+0x34>
   10104:	00000517          	auipc	a0,0x0
   10108:	37050513          	addi	a0,a0,880 # 10474 <__libc_fini_array>
   1010c:	354000ef          	jal	ra,10460 <atexit>
   10110:	0b8000ef          	jal	ra,101c8 <__libc_init_array>
   10114:	00012503          	lw	a0,0(sp)
   10118:	00410593          	addi	a1,sp,4
   1011c:	00000613          	li	a2,0
   10120:	06c000ef          	jal	ra,1018c <main>
   10124:	f71ff06f          	j	10094 <exit>

00010128 <__do_global_dtors_aux>:
   10128:	ff010113          	addi	sp,sp,-16
   1012c:	00812423          	sw	s0,8(sp)
   10130:	c341c783          	lbu	a5,-972(gp) # 119e4 <completed.1>
   10134:	00112623          	sw	ra,12(sp)
   10138:	02079263          	bnez	a5,1015c <__do_global_dtors_aux+0x34>
   1013c:	00000793          	li	a5,0
   10140:	00078a63          	beqz	a5,10154 <__do_global_dtors_aux+0x2c>
   10144:	00011537          	lui	a0,0x11
   10148:	5a050513          	addi	a0,a0,1440 # 115a0 <__FRAME_END__>
   1014c:	00000097          	auipc	ra,0x0
   10150:	000000e7          	jalr	zero # 0 <exit-0x10094>
   10154:	00100793          	li	a5,1
   10158:	c2f18a23          	sb	a5,-972(gp) # 119e4 <completed.1>
   1015c:	00c12083          	lw	ra,12(sp)
   10160:	00812403          	lw	s0,8(sp)
   10164:	01010113          	addi	sp,sp,16
   10168:	00008067          	ret

0001016c <frame_dummy>:
   1016c:	00000793          	li	a5,0
   10170:	00078c63          	beqz	a5,10188 <frame_dummy+0x1c>
   10174:	00011537          	lui	a0,0x11
   10178:	c3818593          	addi	a1,gp,-968 # 119e8 <object.0>
   1017c:	5a050513          	addi	a0,a0,1440 # 115a0 <__FRAME_END__>
   10180:	00000317          	auipc	t1,0x0
   10184:	00000067          	jr	zero # 0 <exit-0x10094>
   10188:	00008067          	ret

0001018c <main>:
   1018c:	fe010113          	addi	sp,sp,-32
   10190:	00812e23          	sw	s0,28(sp)
   10194:	02010413          	addi	s0,sp,32
   10198:	02000793          	li	a5,32
   1019c:	fef42623          	sw	a5,-20(s0)
   101a0:	fec42783          	lw	a5,-20(s0)
   101a4:	00579793          	slli	a5,a5,0x5
   101a8:	fef42423          	sw	a5,-24(s0)
   101ac:	fe842783          	lw	a5,-24(s0)
   101b0:	fef42223          	sw	a5,-28(s0)
   101b4:	00000793          	li	a5,0
   101b8:	00078513          	mv	a0,a5
   101bc:	01c12403          	lw	s0,28(sp)
   101c0:	02010113          	addi	sp,sp,32
   101c4:	00008067          	ret

000101c8 <__libc_init_array>:
   101c8:	ff010113          	addi	sp,sp,-16
   101cc:	00812423          	sw	s0,8(sp)
   101d0:	000117b7          	lui	a5,0x11
   101d4:	00011437          	lui	s0,0x11
   101d8:	01212023          	sw	s2,0(sp)
   101dc:	5a478793          	addi	a5,a5,1444 # 115a4 <__init_array_start>
   101e0:	5a440713          	addi	a4,s0,1444 # 115a4 <__init_array_start>
   101e4:	00112623          	sw	ra,12(sp)
   101e8:	00912223          	sw	s1,4(sp)
   101ec:	40e78933          	sub	s2,a5,a4
   101f0:	02e78263          	beq	a5,a4,10214 <__libc_init_array+0x4c>
   101f4:	40295913          	srai	s2,s2,0x2
   101f8:	5a440413          	addi	s0,s0,1444
   101fc:	00000493          	li	s1,0
   10200:	00042783          	lw	a5,0(s0)
   10204:	00148493          	addi	s1,s1,1
   10208:	00440413          	addi	s0,s0,4
   1020c:	000780e7          	jalr	a5
   10210:	ff24e8e3          	bltu	s1,s2,10200 <__libc_init_array+0x38>
   10214:	00011437          	lui	s0,0x11
   10218:	000117b7          	lui	a5,0x11
   1021c:	5ac78793          	addi	a5,a5,1452 # 115ac <__do_global_dtors_aux_fini_array_entry>
   10220:	5a440713          	addi	a4,s0,1444 # 115a4 <__init_array_start>
   10224:	40e78933          	sub	s2,a5,a4
   10228:	40295913          	srai	s2,s2,0x2
   1022c:	02e78063          	beq	a5,a4,1024c <__libc_init_array+0x84>
   10230:	5a440413          	addi	s0,s0,1444
   10234:	00000493          	li	s1,0
   10238:	00042783          	lw	a5,0(s0)
   1023c:	00148493          	addi	s1,s1,1
   10240:	00440413          	addi	s0,s0,4
   10244:	000780e7          	jalr	a5
   10248:	ff24e8e3          	bltu	s1,s2,10238 <__libc_init_array+0x70>
   1024c:	00c12083          	lw	ra,12(sp)
   10250:	00812403          	lw	s0,8(sp)
   10254:	00412483          	lw	s1,4(sp)
   10258:	00012903          	lw	s2,0(sp)
   1025c:	01010113          	addi	sp,sp,16
   10260:	00008067          	ret

00010264 <memset>:
   10264:	00f00313          	li	t1,15
   10268:	00050713          	mv	a4,a0
   1026c:	02c37e63          	bgeu	t1,a2,102a8 <memset+0x44>
   10270:	00f77793          	andi	a5,a4,15
   10274:	0a079063          	bnez	a5,10314 <memset+0xb0>
   10278:	08059263          	bnez	a1,102fc <memset+0x98>
   1027c:	ff067693          	andi	a3,a2,-16
   10280:	00f67613          	andi	a2,a2,15
   10284:	00e686b3          	add	a3,a3,a4
   10288:	00b72023          	sw	a1,0(a4)
   1028c:	00b72223          	sw	a1,4(a4)
   10290:	00b72423          	sw	a1,8(a4)
   10294:	00b72623          	sw	a1,12(a4)
   10298:	01070713          	addi	a4,a4,16
   1029c:	fed766e3          	bltu	a4,a3,10288 <memset+0x24>
   102a0:	00061463          	bnez	a2,102a8 <memset+0x44>
   102a4:	00008067          	ret
   102a8:	40c306b3          	sub	a3,t1,a2
   102ac:	00269693          	slli	a3,a3,0x2
   102b0:	00000297          	auipc	t0,0x0
   102b4:	005686b3          	add	a3,a3,t0
   102b8:	00c68067          	jr	12(a3)
   102bc:	00b70723          	sb	a1,14(a4)
   102c0:	00b706a3          	sb	a1,13(a4)
   102c4:	00b70623          	sb	a1,12(a4)
   102c8:	00b705a3          	sb	a1,11(a4)
   102cc:	00b70523          	sb	a1,10(a4)
   102d0:	00b704a3          	sb	a1,9(a4)
   102d4:	00b70423          	sb	a1,8(a4)
   102d8:	00b703a3          	sb	a1,7(a4)
   102dc:	00b70323          	sb	a1,6(a4)
   102e0:	00b702a3          	sb	a1,5(a4)
   102e4:	00b70223          	sb	a1,4(a4)
   102e8:	00b701a3          	sb	a1,3(a4)
   102ec:	00b70123          	sb	a1,2(a4)
   102f0:	00b700a3          	sb	a1,1(a4)
   102f4:	00b70023          	sb	a1,0(a4)
   102f8:	00008067          	ret
   102fc:	0ff5f593          	zext.b	a1,a1
   10300:	00859693          	slli	a3,a1,0x8
   10304:	00d5e5b3          	or	a1,a1,a3
   10308:	01059693          	slli	a3,a1,0x10
   1030c:	00d5e5b3          	or	a1,a1,a3
   10310:	f6dff06f          	j	1027c <memset+0x18>
   10314:	00279693          	slli	a3,a5,0x2
   10318:	00000297          	auipc	t0,0x0
   1031c:	005686b3          	add	a3,a3,t0
   10320:	00008293          	mv	t0,ra
   10324:	fa0680e7          	jalr	-96(a3)
   10328:	00028093          	mv	ra,t0
   1032c:	ff078793          	addi	a5,a5,-16
   10330:	40f70733          	sub	a4,a4,a5
   10334:	00f60633          	add	a2,a2,a5
   10338:	f6c378e3          	bgeu	t1,a2,102a8 <memset+0x44>
   1033c:	f3dff06f          	j	10278 <memset+0x14>

00010340 <__call_exitprocs>:
   10340:	fd010113          	addi	sp,sp,-48
   10344:	01412c23          	sw	s4,24(sp)
   10348:	c281aa03          	lw	s4,-984(gp) # 119d8 <_global_impure_ptr>
   1034c:	03212023          	sw	s2,32(sp)
   10350:	02112623          	sw	ra,44(sp)
   10354:	148a2903          	lw	s2,328(s4)
   10358:	02812423          	sw	s0,40(sp)
   1035c:	02912223          	sw	s1,36(sp)
   10360:	01312e23          	sw	s3,28(sp)
   10364:	01512a23          	sw	s5,20(sp)
   10368:	01612823          	sw	s6,16(sp)
   1036c:	01712623          	sw	s7,12(sp)
   10370:	01812423          	sw	s8,8(sp)
   10374:	04090063          	beqz	s2,103b4 <__call_exitprocs+0x74>
   10378:	00050b13          	mv	s6,a0
   1037c:	00058b93          	mv	s7,a1
   10380:	00100a93          	li	s5,1
   10384:	fff00993          	li	s3,-1
   10388:	00492483          	lw	s1,4(s2)
   1038c:	fff48413          	addi	s0,s1,-1
   10390:	02044263          	bltz	s0,103b4 <__call_exitprocs+0x74>
   10394:	00249493          	slli	s1,s1,0x2
   10398:	009904b3          	add	s1,s2,s1
   1039c:	040b8463          	beqz	s7,103e4 <__call_exitprocs+0xa4>
   103a0:	1044a783          	lw	a5,260(s1)
   103a4:	05778063          	beq	a5,s7,103e4 <__call_exitprocs+0xa4>
   103a8:	fff40413          	addi	s0,s0,-1
   103ac:	ffc48493          	addi	s1,s1,-4
   103b0:	ff3416e3          	bne	s0,s3,1039c <__call_exitprocs+0x5c>
   103b4:	02c12083          	lw	ra,44(sp)
   103b8:	02812403          	lw	s0,40(sp)
   103bc:	02412483          	lw	s1,36(sp)
   103c0:	02012903          	lw	s2,32(sp)
   103c4:	01c12983          	lw	s3,28(sp)
   103c8:	01812a03          	lw	s4,24(sp)
   103cc:	01412a83          	lw	s5,20(sp)
   103d0:	01012b03          	lw	s6,16(sp)
   103d4:	00c12b83          	lw	s7,12(sp)
   103d8:	00812c03          	lw	s8,8(sp)
   103dc:	03010113          	addi	sp,sp,48
   103e0:	00008067          	ret
   103e4:	00492783          	lw	a5,4(s2)
   103e8:	0044a683          	lw	a3,4(s1)
   103ec:	fff78793          	addi	a5,a5,-1
   103f0:	04878e63          	beq	a5,s0,1044c <__call_exitprocs+0x10c>
   103f4:	0004a223          	sw	zero,4(s1)
   103f8:	fa0688e3          	beqz	a3,103a8 <__call_exitprocs+0x68>
   103fc:	18892783          	lw	a5,392(s2)
   10400:	008a9733          	sll	a4,s5,s0
   10404:	00492c03          	lw	s8,4(s2)
   10408:	00f777b3          	and	a5,a4,a5
   1040c:	02079263          	bnez	a5,10430 <__call_exitprocs+0xf0>
   10410:	000680e7          	jalr	a3
   10414:	00492703          	lw	a4,4(s2)
   10418:	148a2783          	lw	a5,328(s4)
   1041c:	01871463          	bne	a4,s8,10424 <__call_exitprocs+0xe4>
   10420:	f92784e3          	beq	a5,s2,103a8 <__call_exitprocs+0x68>
   10424:	f80788e3          	beqz	a5,103b4 <__call_exitprocs+0x74>
   10428:	00078913          	mv	s2,a5
   1042c:	f5dff06f          	j	10388 <__call_exitprocs+0x48>
   10430:	18c92783          	lw	a5,396(s2)
   10434:	0844a583          	lw	a1,132(s1)
   10438:	00f77733          	and	a4,a4,a5
   1043c:	00071c63          	bnez	a4,10454 <__call_exitprocs+0x114>
   10440:	000b0513          	mv	a0,s6
   10444:	000680e7          	jalr	a3
   10448:	fcdff06f          	j	10414 <__call_exitprocs+0xd4>
   1044c:	00892223          	sw	s0,4(s2)
   10450:	fa9ff06f          	j	103f8 <__call_exitprocs+0xb8>
   10454:	00058513          	mv	a0,a1
   10458:	000680e7          	jalr	a3
   1045c:	fb9ff06f          	j	10414 <__call_exitprocs+0xd4>

00010460 <atexit>:
   10460:	00050593          	mv	a1,a0
   10464:	00000693          	li	a3,0
   10468:	00000613          	li	a2,0
   1046c:	00000513          	li	a0,0
   10470:	0600006f          	j	104d0 <__register_exitproc>

00010474 <__libc_fini_array>:
   10474:	ff010113          	addi	sp,sp,-16
   10478:	00812423          	sw	s0,8(sp)
   1047c:	000117b7          	lui	a5,0x11
   10480:	00011437          	lui	s0,0x11
   10484:	5ac78793          	addi	a5,a5,1452 # 115ac <__do_global_dtors_aux_fini_array_entry>
   10488:	5b040413          	addi	s0,s0,1456 # 115b0 <impure_data>
   1048c:	40f40433          	sub	s0,s0,a5
   10490:	00912223          	sw	s1,4(sp)
   10494:	00112623          	sw	ra,12(sp)
   10498:	40245493          	srai	s1,s0,0x2
   1049c:	02048063          	beqz	s1,104bc <__libc_fini_array+0x48>
   104a0:	ffc40413          	addi	s0,s0,-4
   104a4:	00f40433          	add	s0,s0,a5
   104a8:	00042783          	lw	a5,0(s0)
   104ac:	fff48493          	addi	s1,s1,-1
   104b0:	ffc40413          	addi	s0,s0,-4
   104b4:	000780e7          	jalr	a5
   104b8:	fe0498e3          	bnez	s1,104a8 <__libc_fini_array+0x34>
   104bc:	00c12083          	lw	ra,12(sp)
   104c0:	00812403          	lw	s0,8(sp)
   104c4:	00412483          	lw	s1,4(sp)
   104c8:	01010113          	addi	sp,sp,16
   104cc:	00008067          	ret

000104d0 <__register_exitproc>:
   104d0:	c281a703          	lw	a4,-984(gp) # 119d8 <_global_impure_ptr>
   104d4:	14872783          	lw	a5,328(a4)
   104d8:	04078c63          	beqz	a5,10530 <__register_exitproc+0x60>
   104dc:	0047a703          	lw	a4,4(a5)
   104e0:	01f00813          	li	a6,31
   104e4:	06e84e63          	blt	a6,a4,10560 <__register_exitproc+0x90>
   104e8:	00271813          	slli	a6,a4,0x2
   104ec:	02050663          	beqz	a0,10518 <__register_exitproc+0x48>
   104f0:	01078333          	add	t1,a5,a6
   104f4:	08c32423          	sw	a2,136(t1) # 10208 <__libc_init_array+0x40>
   104f8:	1887a883          	lw	a7,392(a5)
   104fc:	00100613          	li	a2,1
   10500:	00e61633          	sll	a2,a2,a4
   10504:	00c8e8b3          	or	a7,a7,a2
   10508:	1917a423          	sw	a7,392(a5)
   1050c:	10d32423          	sw	a3,264(t1)
   10510:	00200693          	li	a3,2
   10514:	02d50463          	beq	a0,a3,1053c <__register_exitproc+0x6c>
   10518:	00170713          	addi	a4,a4,1
   1051c:	00e7a223          	sw	a4,4(a5)
   10520:	010787b3          	add	a5,a5,a6
   10524:	00b7a423          	sw	a1,8(a5)
   10528:	00000513          	li	a0,0
   1052c:	00008067          	ret
   10530:	14c70793          	addi	a5,a4,332
   10534:	14f72423          	sw	a5,328(a4)
   10538:	fa5ff06f          	j	104dc <__register_exitproc+0xc>
   1053c:	18c7a683          	lw	a3,396(a5)
   10540:	00170713          	addi	a4,a4,1
   10544:	00e7a223          	sw	a4,4(a5)
   10548:	00c6e6b3          	or	a3,a3,a2
   1054c:	18d7a623          	sw	a3,396(a5)
   10550:	010787b3          	add	a5,a5,a6
   10554:	00b7a423          	sw	a1,8(a5)
   10558:	00000513          	li	a0,0
   1055c:	00008067          	ret
   10560:	fff00513          	li	a0,-1
   10564:	00008067          	ret

00010568 <_exit>:
   10568:	05d00893          	li	a7,93
   1056c:	00000073          	ecall
   10570:	00054463          	bltz	a0,10578 <_exit+0x10>
   10574:	0000006f          	j	10574 <_exit+0xc>
   10578:	ff010113          	addi	sp,sp,-16
   1057c:	00812423          	sw	s0,8(sp)
   10580:	00050413          	mv	s0,a0
   10584:	00112623          	sw	ra,12(sp)
   10588:	40800433          	neg	s0,s0
   1058c:	00c000ef          	jal	ra,10598 <__errno>
   10590:	00852023          	sw	s0,0(a0)
   10594:	0000006f          	j	10594 <_exit+0x2c>

00010598 <__errno>:
   10598:	c301a503          	lw	a0,-976(gp) # 119e0 <_impure_ptr>
   1059c:	00008067          	ret

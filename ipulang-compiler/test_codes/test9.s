	.text
	.file	"main"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movl	$0, -4(%rbp)
	movb	$1, %al
	testb	%al, %al
	jne	.LBB0_2
# %bb.1:                                # %label4
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$1, -16(%rax)
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	$2, -16(%rcx)
	movl	-16(%rax), %eax
	addl	%eax, %eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	jmp	.LBB0_4
.LBB0_2:                                # %label5
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	$0, -16(%rcx)
	testb	%al, %al
	jne	.LBB0_5
# %bb.3:                                # %label17
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$3, -16(%rax)
	movl	$3, %eax
	jmp	.LBB0_4
.LBB0_5:                                # %label18
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$4, -16(%rax)
	movl	$4, %eax
.LBB0_4:                                # %label17
	movq	%rbp, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits

// @counter に 16 をセット
@16 
D=A
@counter
M=D
// @writeaddress を SCREEN の最初にする
@SCREEN
D=A
@writeaddress
M=D
    (LOOP)
// @writeaddress を 1 ビットで埋める    
@writeaddress
A=M
M=-1

// @writeaddress を 32 すすめる
@writeaddress
D=M
@32
D=D+A

// @writeaddress に D を保存
@writeaddress
M=D

// @counter から 1 減らす
@counter
MD=M-1
// loop まで戻る
@LOOP
D;JGT
    (INF_LOOP)
@INF_LOOP
0;JMP

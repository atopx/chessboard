<script setup lang="ts">

import { emit, listen } from "@tauri-apps/api/event";
import { onMounted } from "vue";


type Piece = {
  key: string,
  position: string,
}

type Move = {
  from: string,
  to: string,
}

const startpos: Piece[] = [
  {key: "R", position: "a0"},
  {key: "N", position: "b0"},
  {key: "B", position: "c0"},
  {key: "A", position: "d0"},
  {key: "K", position: "e0"},
  {key: "A", position: "f0"},
  {key: "B", position: "g0"},
  {key: "N", position: "h0"},
  {key: "R", position: "i0"},
  {key: "C", position: "b2"},
  {key: "C", position: "h2"},
  {key: "P", position: "a3"},
  {key: "P", position: "c3"},
  {key: "P", position: "e3"},
  {key: "P", position: "g3"},
  {key: "P", position: "i3"},
  {key: "r", position: "a9"},
  {key: "n", position: "b9"},
  {key: "b", position: "c9"},
  {key: "a", position: "d9"},
  {key: "k", position: "e9"},
  {key: "a", position: "f9"},
  {key: "b", position: "g9"},
  {key: "n", position: "h9"},
  {key: "r", position: "i9"},
  {key: "c", position: "b7"},
  {key: "c", position: "h7"},
  {key: "p", position: "a6"},
  {key: "p", position: "c6"},
  {key: "p", position: "e6"},
  {key: "p", position: "g6"},
  {key: "p", position: "i6"},
];

// 设置棋子的函数
async function setPiecesOnBoard(pieces: Piece[]) {
  // 遍历棋子位置，并在棋盘上创建对应的棋子元素
  for (let index = 0; index < pieces.length; index++) {
    const piece = pieces[index];
    // 找到坐标并添加棋子
    document.getElementById(piece.position)?.firstElementChild?.classList.add(`piece-${piece.key}`);
  }
  document.getElementById("a0")?.classList.add("b-select");
}

onMounted(async () => {
  await emit('position', startpos)
})

// 移动棋子
async function setPieceMove(move: Move) {
  // document.getElementById(move.from)?.classList.keys().
}


listen('position', async (event) => {
  let pos  = event.payload as Piece[];
  await setPiecesOnBoard(pos);
})

listen('move', async (event) => {
  let move = event.payload as Move;
  // await
});


</script>

<template>
  <div id="chessboard">
    <div id="a9" class="piece-wrap"><span class="piece"></span></div>
    <div id="b9" class="piece-wrap"><span class="piece"></span></div>
    <div id="c9" class="piece-wrap"><span class="piece"></span></div>
    <div id="d9" class="piece-wrap"><span class="piece"></span></div>
    <div id="e9" class="piece-wrap"><span class="piece"></span></div>
    <div id="f9" class="piece-wrap"><span class="piece"></span></div>
    <div id="g9" class="piece-wrap"><span class="piece"></span></div>
    <div id="h9" class="piece-wrap"><span class="piece"></span></div>
    <div id="i9" class="piece-wrap"><span class="piece"></span></div>
    <div id="a8" class="piece-wrap"><span class="piece"></span></div>
    <div id="b8" class="piece-wrap"><span class="piece"></span></div>
    <div id="c8" class="piece-wrap"><span class="piece"></span></div>
    <div id="d8" class="piece-wrap"><span class="piece"></span></div>
    <div id="e8" class="piece-wrap"><span class="piece"></span></div>
    <div id="f8" class="piece-wrap"><span class="piece"></span></div>
    <div id="g8" class="piece-wrap"><span class="piece"></span></div>
    <div id="h8" class="piece-wrap"><span class="piece"></span></div>
    <div id="i8" class="piece-wrap"><span class="piece"></span></div>
    <div id="a7" class="piece-wrap"><span class="piece"></span></div>
    <div id="b7" class="piece-wrap"><span class="piece"></span></div>
    <div id="c7" class="piece-wrap"><span class="piece"></span></div>
    <div id="d7" class="piece-wrap"><span class="piece"></span></div>
    <div id="e7" class="piece-wrap"><span class="piece"></span></div>
    <div id="f7" class="piece-wrap"><span class="piece"></span></div>
    <div id="g7" class="piece-wrap"><span class="piece"></span></div>
    <div id="h7" class="piece-wrap"><span class="piece"></span></div>
    <div id="i7" class="piece-wrap"><span class="piece"></span></div>
    <div id="a6" class="piece-wrap"><span class="piece"></span></div>
    <div id="b6" class="piece-wrap"><span class="piece"></span></div>
    <div id="c6" class="piece-wrap"><span class="piece"></span></div>
    <div id="d6" class="piece-wrap"><span class="piece"></span></div>
    <div id="e6" class="piece-wrap"><span class="piece"></span></div>
    <div id="f6" class="piece-wrap"><span class="piece"></span></div>
    <div id="g6" class="piece-wrap"><span class="piece"></span></div>
    <div id="h6" class="piece-wrap"><span class="piece"></span></div>
    <div id="i6" class="piece-wrap"><span class="piece"></span></div>
    <div id="a5" class="piece-wrap"><span class="piece"></span></div>
    <div id="b5" class="piece-wrap"><span class="piece"></span></div>
    <div id="c5" class="piece-wrap"><span class="piece"></span></div>
    <div id="d5" class="piece-wrap"><span class="piece"></span></div>
    <div id="e5" class="piece-wrap"><span class="piece"></span></div>
    <div id="f5" class="piece-wrap"><span class="piece"></span></div>
    <div id="g5" class="piece-wrap"><span class="piece"></span></div>
    <div id="h5" class="piece-wrap"><span class="piece"></span></div>
    <div id="i5" class="piece-wrap"><span class="piece"></span></div>
    <div id="a4" class="piece-wrap"><span class="piece"></span></div>
    <div id="b4" class="piece-wrap"><span class="piece"></span></div>
    <div id="c4" class="piece-wrap"><span class="piece"></span></div>
    <div id="d4" class="piece-wrap"><span class="piece"></span></div>
    <div id="e4" class="piece-wrap"><span class="piece"></span></div>
    <div id="f4" class="piece-wrap"><span class="piece"></span></div>
    <div id="g4" class="piece-wrap"><span class="piece"></span></div>
    <div id="h4" class="piece-wrap"><span class="piece"></span></div>
    <div id="i4" class="piece-wrap"><span class="piece"></span></div>
    <div id="a3" class="piece-wrap"><span class="piece"></span></div>
    <div id="b3" class="piece-wrap"><span class="piece"></span></div>
    <div id="c3" class="piece-wrap"><span class="piece"></span></div>
    <div id="d3" class="piece-wrap"><span class="piece"></span></div>
    <div id="e3" class="piece-wrap"><span class="piece"></span></div>
    <div id="f3" class="piece-wrap"><span class="piece"></span></div>
    <div id="g3" class="piece-wrap"><span class="piece"></span></div>
    <div id="h3" class="piece-wrap"><span class="piece"></span></div>
    <div id="i3" class="piece-wrap"><span class="piece"></span></div>
    <div id="a2" class="piece-wrap"><span class="piece"></span></div>
    <div id="b2" class="piece-wrap"><span class="piece"></span></div>
    <div id="c2" class="piece-wrap"><span class="piece"></span></div>
    <div id="d2" class="piece-wrap"><span class="piece"></span></div>
    <div id="e2" class="piece-wrap"><span class="piece"></span></div>
    <div id="f2" class="piece-wrap"><span class="piece"></span></div>
    <div id="g2" class="piece-wrap"><span class="piece"></span></div>
    <div id="h2" class="piece-wrap"><span class="piece"></span></div>
    <div id="i2" class="piece-wrap"><span class="piece"></span></div>
    <div id="a1" class="piece-wrap"><span class="piece"></span></div>
    <div id="b1" class="piece-wrap"><span class="piece"></span></div>
    <div id="c1" class="piece-wrap"><span class="piece"></span></div>
    <div id="d1" class="piece-wrap"><span class="piece"></span></div>
    <div id="e1" class="piece-wrap"><span class="piece"></span></div>
    <div id="f1" class="piece-wrap"><span class="piece"></span></div>
    <div id="g1" class="piece-wrap"><span class="piece"></span></div>
    <div id="h1" class="piece-wrap"><span class="piece"></span></div>
    <div id="i1" class="piece-wrap"><span class="piece"></span></div>
    <div id="a0" class="piece-wrap"><span class="piece"></span></div>
    <div id="b0" class="piece-wrap"><span class="piece"></span></div>
    <div id="c0" class="piece-wrap"><span class="piece"></span></div>
    <div id="d0" class="piece-wrap"><span class="piece"></span></div>
    <div id="e0" class="piece-wrap"><span class="piece"></span></div>
    <div id="f0" class="piece-wrap"><span class="piece"></span></div>
    <div id="g0" class="piece-wrap"><span class="piece"></span></div>
    <div id="h0" class="piece-wrap"><span class="piece"></span></div>
    <div id="i0" class="piece-wrap"><span class="piece"></span></div>
  </div>
</template>

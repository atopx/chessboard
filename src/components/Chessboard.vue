<script setup lang="ts">

import { emit, listen } from "@tauri-apps/api/event";
import { onMounted } from "vue";


interface Position {
  piece: string,
  pos: string,
}

interface Changed {
  piece: string,
  from: string,
  to: string,
  camp: string,
}


const startpos: Position[] = [
  { piece: "R", pos: "a0" },
  { piece: "N", pos: "b0" },
  { piece: "B", pos: "c0" },
  { piece: "A", pos: "d0" },
  { piece: "K", pos: "e0" },
  { piece: "A", pos: "f0" },
  { piece: "B", pos: "g0" },
  { piece: "N", pos: "h0" },
  { piece: "R", pos: "i0" },
  { piece: "C", pos: "b2" },
  { piece: "C", pos: "h2" },
  { piece: "P", pos: "a3" },
  { piece: "P", pos: "c3" },
  { piece: "P", pos: "e3" },
  { piece: "P", pos: "g3" },
  { piece: "P", pos: "i3" },
  { piece: "r", pos: "a9" },
  { piece: "n", pos: "b9" },
  { piece: "b", pos: "c9" },
  { piece: "a", pos: "d9" },
  { piece: "k", pos: "e9" },
  { piece: "a", pos: "f9" },
  { piece: "b", pos: "g9" },
  { piece: "n", pos: "h9" },
  { piece: "r", pos: "i9" },
  { piece: "c", pos: "b7" },
  { piece: "c", pos: "h7" },
  { piece: "p", pos: "a6" },
  { piece: "p", pos: "c6" },
  { piece: "p", pos: "e6" },
  { piece: "p", pos: "g6" },
  { piece: "p", pos: "i6" },
];

// 设置棋子的函数
async function setPiecesOnBoard(pieces: Position[]) {
  // 遍历棋子位置，并在棋盘上创建对应的棋子元素
  for (let index = 0; index < pieces.length; index++) {
    const record = pieces[index];
    console.log(record)
    let ele = document.getElementById(record.pos)?.firstElementChild;
    // 移除坐标原棋子
    ele?.classList.forEach(cls => {
        if (cls != "piece") {
          console.log(`remove ${record.pos} -> ${cls}`)
          ele?.classList.remove(cls)
        }
      });
    // 添加新棋子
    if (record.piece != " ") {
      ele?.classList.add(`piece-${record.piece}`);
    }
  }
}

onMounted(async () => {
  await emit('position', startpos)
})



listen('position', async (event) => {
  let pos = event.payload as Position[];
  await setPiecesOnBoard(pos);
})

listen('move', async (event) => {
  let change = event.payload as Changed;
  let token = `piece-${change.piece}`;
  console.log(change);

  // 原坐标移除棋子
  document.getElementById(change.from)?.firstElementChild?.classList.remove(token);

  // 移除目标坐标棋子
  let ele = document.getElementById(change.to)?.firstElementChild;
  ele?.classList.forEach(cls => {
    if (cls != "piece") {
      ele?.classList.remove(cls)
    }
  });

  // 目标坐标添加棋子
  document.getElementById(change.to)?.firstElementChild?.classList.add(token);
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
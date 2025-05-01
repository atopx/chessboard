<script setup lang="ts">

import { emit, listen } from "@tauri-apps/api/event";
import { computed, onMounted, ref } from "vue";


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
        let ele = document.getElementById(record.pos)?.firstElementChild;

        // 移除 select
        document.querySelectorAll(".b-select").forEach(element => {
            element.classList.remove("b-select")
        });

        // 移除坐标原棋子
        ele?.classList.forEach(cls => {
            if (cls != "piece") {
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

const mirror = ref(false);

const wrappedItems = computed(() => {
    if (mirror.value) {
        return [{ id: 'i0' }, { id: 'h0' }, { id: 'g0' }, { id: 'f0' }, { id: 'e0' }, { id: 'd0' }, { id: 'c0' }, { id: 'b0' }, { id: 'a0' }, { id: 'i1' }, { id: 'h1' }, { id: 'g1' }, { id: 'f1' }, { id: 'e1' }, { id: 'd1' }, { id: 'c1' }, { id: 'b1' }, { id: 'a1' }, { id: 'i2' }, { id: 'h2' }, { id: 'g2' }, { id: 'f2' }, { id: 'e2' }, { id: 'd2' }, { id: 'c2' }, { id: 'b2' }, { id: 'a2' }, { id: 'i3' }, { id: 'h3' }, { id: 'g3' }, { id: 'f3' }, { id: 'e3' }, { id: 'd3' }, { id: 'c3' }, { id: 'b3' }, { id: 'a3' }, { id: 'i4' }, { id: 'h4' }, { id: 'g4' }, { id: 'f4' }, { id: 'e4' }, { id: 'd4' }, { id: 'c4' }, { id: 'b4' }, { id: 'a4' }, { id: 'i5' }, { id: 'h5' }, { id: 'g5' }, { id: 'f5' }, { id: 'e5' }, { id: 'd5' }, { id: 'c5' }, { id: 'b5' }, { id: 'a5' }, { id: 'i6' }, { id: 'h6' }, { id: 'g6' }, { id: 'f6' }, { id: 'e6' }, { id: 'd6' }, { id: 'c6' }, { id: 'b6' }, { id: 'a6' }, { id: 'i7' }, { id: 'h7' }, { id: 'g7' }, { id: 'f7' }, { id: 'e7' }, { id: 'd7' }, { id: 'c7' }, { id: 'b7' }, { id: 'a7' }, { id: 'i8' }, { id: 'h8' }, { id: 'g8' }, { id: 'f8' }, { id: 'e8' }, { id: 'd8' }, { id: 'c8' }, { id: 'b8' }, { id: 'a8' }, { id: 'i9' }, { id: 'h9' }, { id: 'g9' }, { id: 'f9' }, { id: 'e9' }, { id: 'd9' }, { id: 'c9' }, { id: 'b9' }, { id: 'a9' }];

    } else {
        return [{ id: 'a9' }, { id: 'b9' }, { id: 'c9' }, { id: 'd9' }, { id: 'e9' }, { id: 'f9' }, { id: 'g9' }, { id: 'h9' }, { id: 'i9' }, { id: 'a8' }, { id: 'b8' }, { id: 'c8' }, { id: 'd8' }, { id: 'e8' }, { id: 'f8' }, { id: 'g8' }, { id: 'h8' }, { id: 'i8' }, { id: 'a7' }, { id: 'b7' }, { id: 'c7' }, { id: 'd7' }, { id: 'e7' }, { id: 'f7' }, { id: 'g7' }, { id: 'h7' }, { id: 'i7' }, { id: 'a6' }, { id: 'b6' }, { id: 'c6' }, { id: 'd6' }, { id: 'e6' }, { id: 'f6' }, { id: 'g6' }, { id: 'h6' }, { id: 'i6' }, { id: 'a5' }, { id: 'b5' }, { id: 'c5' }, { id: 'd5' }, { id: 'e5' }, { id: 'f5' }, { id: 'g5' }, { id: 'h5' }, { id: 'i5' }, { id: 'a4' }, { id: 'b4' }, { id: 'c4' }, { id: 'd4' }, { id: 'e4' }, { id: 'f4' }, { id: 'g4' }, { id: 'h4' }, { id: 'i4' }, { id: 'a3' }, { id: 'b3' }, { id: 'c3' }, { id: 'd3' }, { id: 'e3' }, { id: 'f3' }, { id: 'g3' }, { id: 'h3' }, { id: 'i3' }, { id: 'a2' }, { id: 'b2' }, { id: 'c2' }, { id: 'd2' }, { id: 'e2' }, { id: 'f2' }, { id: 'g2' }, { id: 'h2' }, { id: 'i2' }, { id: 'a1' }, { id: 'b1' }, { id: 'c1' }, { id: 'd1' }, { id: 'e1' }, { id: 'f1' }, { id: 'g1' }, { id: 'h1' }, { id: 'i1' }, { id: 'a0' }, { id: 'b0' }, { id: 'c0' }, { id: 'd0' }, { id: 'e0' }, { id: 'f0' }, { id: 'g0' }, { id: 'h0' }, { id: 'i0' }];
    }
});

listen('mirror', async (event) => {
    mirror.value = event.payload as boolean;
})

listen('position', async (event) => {
    let pos = event.payload as Position[];
    await setPiecesOnBoard(pos);
})

listen('move', async (event) => {
    let change = event.payload as Changed;
    let token = `piece-${change.piece}`;

    // 移除 select
    document.querySelectorAll(".b-select").forEach(element => {
        element.classList.remove("b-select")
    });

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
        <div v-for="(item, _) in wrappedItems" :key="item.id" :id="item.id" class="piece-wrap"><span
                class="piece"></span></div>
    </div>
</template>
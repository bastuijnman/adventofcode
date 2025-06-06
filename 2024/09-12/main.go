package main

import (
	"fmt"
	"os"
	"strconv"
)

type Block struct {
	id        int
	size      int
	file      bool
	content   []int
	writeHead int
}

func getBlockContent(block Block) []int {
	content := make([]int, block.size)
	for i := range content {
		content[i] = block.id
	}
	return content
}

func getFirstWritableBlockIndex(blocks []Block) (int, error) {
	for idx, block := range blocks {
		if len(block.content) != block.size && !block.file {
			return idx, nil
		}
	}
	return 0, fmt.Errorf("No writeable block found")
}

func getFirstFillableBlockIndex(blocks []Block, size int) (int, error) {
	for idx, block := range blocks {
		if block.size-len(block.content) >= size {
			return idx, nil
		}
	}
	return 0, fmt.Errorf("No writeable block found")
}

func getTotalBlockSequence(blocks []Block) []int {
	var sequence []int
	for _, block := range blocks {
		fill := make([]int, block.size-len(block.content))
		sequence = append(sequence, block.content...)
		sequence = append(sequence, fill...)
	}
	return sequence
}

func defragBlocks(blocks []Block, fillEntireBlock bool) []Block {
	for i := len(blocks) - 1; i > 0; i-- {
		block := blocks[i]

		// if it's already an empty block we can skip it
		if !block.file {
			continue
		}

		for len(block.content) != 0 {

			var writeBlockIndex int
			var writeBlockErr error
			if !fillEntireBlock {
				writeBlockIndex, writeBlockErr = getFirstWritableBlockIndex(blocks)
			} else {
				writeBlockIndex, writeBlockErr = getFirstFillableBlockIndex(blocks, len(block.content))
			}

			if writeBlockErr != nil || writeBlockIndex > i {
				break
			}

			writeBlock := blocks[writeBlockIndex]
			writes := min(writeBlock.size-writeBlock.writeHead, len(block.content))
			writeBlock.content = append(writeBlock.content, block.content[:writes]...)
			writeBlock.writeHead = len(writeBlock.content)
			block.content = block.content[writes:]

			// This is definitely not optimal, I'd want to just reference
			// the block but couldn't get it to work...
			blocks[i] = block
			blocks[writeBlockIndex] = writeBlock
		}
	}

	return blocks
}

func getChecksumForBlocks(blocks []Block) int {
	total := 0
	sequence := getTotalBlockSequence(blocks)
	for idx, num := range sequence {
		total += (idx * num)
	}
	return total
}

func main() {

	args := os.Args[1:]
	file := args[0]

	data, err := os.ReadFile(file)
	if err != nil {
		fmt.Println("Error while reading input file")
		return
	}

	var blocks []Block
	for i, n := range string(data) {

		// Ignore newline entry
		if string(n) == "\n" {
			continue
		}
		num, _ := strconv.Atoi(string(n))

		block := Block{
			id:   i / 2,
			size: num,
			file: i%2 == 0,
		}

		if block.file {
			block.content = getBlockContent(block)
			block.writeHead = len(block.content)
		}

		blocks = append(blocks, block)
	}

	// Make sure we retain two unique copies of the input to pass along later
	blocksForPartTwo := make([]Block, len(blocks))
	copy(blocksForPartTwo, blocks)

	blocksPartOne := defragBlocks(blocks, false)
	blocksPartTwo := defragBlocks(blocksForPartTwo, true)

	fmt.Println("Answer part one:", getChecksumForBlocks(blocksPartOne))
	fmt.Println("Answer part two:", getChecksumForBlocks(blocksPartTwo))

}

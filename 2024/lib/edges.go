package lib

type EdgeLocation int

const (
	TopEdge EdgeLocation = iota
	RightEdge
	BottomEdge
	LeftEdge
)

type Edge struct {
	Index    int
	Location EdgeLocation
}

func Edges(index int, cols int, rows int) []Edge {
	var edges []Edge

	// Top
	if index-cols >= 0 {
		edges = append(edges, Edge{
			Index:    index - cols,
			Location: TopEdge,
		})
	}

	// Right
	if (index+1)%cols != 0 {
		edges = append(edges, Edge{
			Index:    index + 1,
			Location: RightEdge,
		})
	}

	// Bottom
	if index+cols < cols*rows {
		edges = append(edges, Edge{
			Index:    index + cols,
			Location: BottomEdge,
		})
	}

	// Left
	if index%cols != 0 {
		edges = append(edges, Edge{
			Index:    index - 1,
			Location: LeftEdge,
		})
	}

	return edges
}

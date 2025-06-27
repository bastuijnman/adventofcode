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
	location EdgeLocation
}

func Edges(index int, cols int, rows int) []Edge {
	var edges []Edge

	// Top
	if index-cols >= 0 {
		edges = append(edges, Edge{
			Index:    index - cols,
			location: TopEdge,
		})
	}

	// Right
	if (index+1)%cols != 0 {
		edges = append(edges, Edge{
			Index:    index + 1,
			location: RightEdge,
		})
	}

	// Bottom
	if index+cols < cols*rows {
		edges = append(edges, Edge{
			Index:    index + cols,
			location: BottomEdge,
		})
	}

	// Left
	if index%cols != 0 {
		edges = append(edges, Edge{
			Index:    index - 1,
			location: LeftEdge,
		})
	}

	return edges
}

POLYHEDRONS: dict[str, int] = {
    "Tetrahedron": 4,
    "Cube": 6,
    "Octahedron": 8,
    "Dodecahedron": 12,
    "Icosahedron": 20,
}

def main() -> None:
    n: int = int(input())
    faces: int = 0

    for _ in range(n):
        faces += POLYHEDRONS[input()]

    print(faces)

if __name__ == "__main__":
    main()
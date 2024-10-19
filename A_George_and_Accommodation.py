def main() -> None:
    rooms: int = int(input())
    free_rooms: int = 0
    
    for room in range(rooms):
        p, q = map(int, input().split())
        if q - p >= 2:
            free_rooms += 1
    print(free_rooms)

if __name__ == "__main__":
    main()
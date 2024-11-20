from collections import Counter

def main() -> None:
    groups_count: int = int(input())
    groups: Counter[int] = Counter(map(int, input().split()))

    cars: int = 0
    cars += groups[4]

    cars += groups[3]
    if groups[1] > groups[3]:
        groups[1] -= groups[3]
    else:
        groups[1] = 0
    
    cars += groups[2] // 2
    if groups[2] % 2 == 1:
        cars += 1
        if groups[1] > 1:
            groups[1] -= 2
        elif groups[1] == 1:
            groups[1] = 0

    cars += groups[1] // 4
    if groups[1] % 4 != 0:
        cars += 1

    print(cars)

if __name__ == "__main__":
    main()
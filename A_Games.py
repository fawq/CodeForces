from collections import Counter

def main() -> None:
    teams: int = int(input())
    host_colors: list[int] = []
    guest_colors: list[int] = []

    for _ in range(teams):
        host_color, guest_color = map(int, input().split())

        host_colors.append(host_color)
        guest_colors.append(guest_color)

    host_colors_counter: Counter[int] = Counter(host_colors)
    guest_colors_counter: Counter[int] = Counter(guest_colors)

    sum: int = 0
    for color in host_colors_counter:
        sum += host_colors_counter[color] * guest_colors_counter[color]

    print(sum)

if __name__ == "__main__":
    main()
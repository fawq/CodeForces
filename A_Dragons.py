def main() -> None:
    strength, number_of_duels = map(int, input().split())
    duels: list[tuple[int, int]] = []

    for _ in range(number_of_duels):
        boss_strength, additional_strength = map(int, input().split())
        duels.append((boss_strength, additional_strength))

    duels.sort(key=lambda duel: duel[0])

    for duel in duels:
        if strength <= duel[0]:
            print("NO")
            break
        strength += duel[1]
    else:
        print("YES")

if __name__ == "__main__":
    main()
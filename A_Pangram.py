def main() -> None:
    _ = input()
    print("YES" if len(set(input().lower())) == 26 else "NO")

if __name__ == "__main__":
    main()
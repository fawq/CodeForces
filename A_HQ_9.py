def main() -> None:
    instructions: str = input()

    for instruction in instructions:
        if instruction in "HQ9":
            print("YES")
            break
    else:
        print("NO")

if __name__ == "__main__":
    main()
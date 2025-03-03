# Gitstrap

`gitstrap` is a CLI tool that helps you initialize a new GitHub repository, blazingly fast!

## Installation

You can install `gitstrap` using `cargo`:

```bash
cargo install gitstrap
```

## Usage

To initialize a new GitHub repository, simply run `gitstrap` in your terminal:

```bash
gitstrap
```

This will prompt you for the remote origin URL (e.g., `https://github.com/ShalevAri/gitstrap.git`) and then run the following commands:

```bash
git init
git add .
git commit -m "initial commit"
git branch -M main
git remote add origin https://github.com/ShalevAri/gitstrap.git
git push -u origin main
```

## License

`gitstrap` is licensed under the GNU Affero General Public License v3.0. See the [LICENSE](LICENSE) file for more information.

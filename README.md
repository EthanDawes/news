# news
Home of my newsletters

**Features:**
- 🖌️ Easily create with liquid templates & mjml for email compatability
- 👁️ Email open analytics with integrated pixel tracking server
- 📋 Subscribe/unsubscribe
- 📤 Automated bulk sending
- ⚙️ Easy to repurpose for yourself, can run entirely in GitHub

## Developing
### Web
1. `pnpm install`
2. `pnpm dev`
3. Deployments are made automaticly on push to GitHub

### Server (optional)
1. Install rust toolchain (rustc, cargo)
2. `cd target/debug`
2. `cargo run` or `cargo build --release`

### Mailer (python script)
1. [Install python](https://www.python.org/downloads/)
2. `pip install dotenv tqdm requests`
3. `cp .sample.env .env`
4. Make & populate `recipients.csv` with format `<name>,<email>`
5. `python3 newsletter-mailer.py` (will confirm before sending)

## Guidelines
- Body images are square & resized to 250x250
- The smallest dimension of the hero image is resized to 512 (use the "fit" option of PowerToys image resizer)
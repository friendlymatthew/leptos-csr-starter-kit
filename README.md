# Leptos CSR with TailwindCSS Starter Template
Unlock rapid development with Leptos, paired seamlessly with TailwindCSS. This template is crafted to save you the hassle of piecing everything together. Developers can focus on what truly matters: building outstanding web applications.


## Quick Start 🚀
### Inspired by `create-react-app`, globally install the [`crate`](https://crates.io/crates/create-leptos-csr-tw):
```bash
cargo add create-leptos-csr-tw
```

### Once installed, you can generate a new Leptos project with TailwindCSS by simply running:
```bash
create-leptos-csr-tw
```
Please follow the prompts to configure your project. It will ask if you want to use Vercel for deployment, and if you choose so, it will generate a `vercel.json` for client-side routing.

### To run the app, simply run both:
```bash
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```
and
```bash
trunk serve --open
```

For more information, check out the [crate's README](https://github.com/friendlymatthew/create-leptos-csr-tw#README)

## Features

### 1. **TailwindCSS Ready**
The project is set up with TailwindCSS based on the recommended configurations from [GreatGreg's guide](https://github.com/leptos-rs/leptos/discussions/125).

### 2. **Vercel Deployment**
To deploy seamlessly on Vercel, the provided `vercel.json` file is set up to handle routing correctly. With this configuration, any unmatched route will serve the `index.html` file, letting the client-side logic handle the routing. This is ideal for single-page applications.

### 3. **Public Assets Management**
Your static assets like images are in good hands. Place them in the `/public` folder and they'll be automatically copied to the `/dist` during compilation. No missed assets after building!

### 4. **Leptos-Use**
Benefit from pre-included [Leptos-Use](https://github.com/Synphonyte/leptos-use/tree/main), your go-to collection of essential Leptos utilities. 


## Conclusion
Whether you're looking to quickly prototype or build a production-ready application, this starter template offers a solid foundation for your Leptos-based projects. Adjust and extend as per your requirements!


## Acknowledgements
A hearty shoutout to [AlphaKeks](https://github.com/AlphaKeks) and [Stefan Todorov](https://github.com/coravacav) for their invaluable suggestions.

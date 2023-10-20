# Leptos CSR with TailwindCSS Starter Template

This template provides a quick setup to start building web applications using the Leptos web framework integrated with TailwindCSS. It also includes configurations for deployment with Vercel and handling public assets.

## Quick Start

For detailed instructions and usage guidelines, refer to the [official manual](https://github.com/friendlymatthew/create-leptos-csr-tw).

## Features

### 1. **TailwindCSS Integration**
The project is set up with TailwindCSS based on the recommended configurations from [GreatGreg's guide](https://github.com/leptos-rs/leptos/discussions/125).

### 2. **Vercel Deployment**
To deploy seamlessly on Vercel, the provided `vercel.json` file is set up to handle routing correctly. With this configuration, any unmatched route will serve the `index.html` file, letting the client-side logic handle the routing. This is ideal for single-page applications.

### 3. **Public Assets Management**
Easily manage and serve static assets like images. Any content in the `/public` folder is automatically copied to the `/dist` folder during the compile process. This ensures that your assets remain accessible when the application is built.

## Conclusion
Whether you're looking to quickly prototype or build a production-ready application, this starter template offers a solid foundation for your Leptos-based projects. Adjust and extend as per your requirements!
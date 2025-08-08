# Welcome to my portfolio project 

## What is this project about?
This project is a showcase of my skills and experiences as a Cloud Engineer / DevOps Engineer in a website portfolio.

## Technologies Used
- 🦀 Rust
    - Actix
    - Tokio
- 🐳 Docker
- 🚀 GitHub Actions
- ☁️ GCP & Proxmox
- 🔧 Terraform

## Why hybrid solution proxmox + GCP?

- Proxmox hosts nginx LXC containers as a reverse proxy for the website. This solution reduces costs for GCP load balancer and improves performance by caching static content.

- GCP Hosts the web application with cloud run. With this setup, I can take advantage of the scalability and flexibility of GCP while keeping costs low with Proxmox.

## Infrastructure as Code

- I use Terraform to manage the infrastructure for both Proxmox and GCP. This allows me to version control my infrastructure and easily reproduce it in different environments.

- Use Proxmox cloud init options.

- Ansible for nginx configuration and setup.

## CI/CD Pipeline

- GitHub Actions for automated testing and deployment. Both for the Proxmox and GCP environments.
- Rust pipeline for building and testing the application.

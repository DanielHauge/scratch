terraform {
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

variable "do_token" {}

provider "digitalocean" {
  token = var.do_token
}

resource "digitalocean_ssh_key" "terraform-deploy-key" {
  name       = "my-ssh-key-used-by-terraform"
  public_key = file("~/.ssh/id_ed25519.pub")
}

resource "digitalocean_droplet" "terraform-test-droplet" {
  name     = "terraform-test-droplet" # The name of the droplet in DigitalOcean
  region   = "fra1"                   # Change to your preferred region
  size     = "s-1vcpu-1gb"            # Choose a plan (check DigitalOcean pricing)
  image    = "ubuntu-24-10-x64"       # Base imagk
  ssh_keys = [digitalocean_ssh_key.terraform-deploy-key.fingerprint]

  provisioner "remote-exec" {
    inline = [
      "sudo apt-get update -y",
      "sudo apt-get install -y ca-certificates curl gnupg lsb-release sudo",
      "sudo curl -fsSL https://get.docker.com -o get-docker.sh",
      "sudo sh get-docker.sh",
    ]
  }

  connection {
    type        = "ssh"
    user        = "root"
    private_key = file("~/.ssh/id_ed25519")
    host        = self.ipv4_address
  }
}

resource "null_resource" "package-docker-image" {
  triggers = {
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command = "docker pull nginx:latest && docker save -o /tmp/my-image.tar nginx"
  }
}

resource "null_resource" "deploy-image" {
  depends_on = [null_resource.package-docker-image]

  provisioner "file" {
    source      = "/tmp/my-image.tar"
    destination = "/tmp/my-image.tar"
  }

  provisioner "remote-exec" {
    inline = [
      "docker load -i /tmp/my-image.tar",
      "docker image ls",
      "docker run -d -p 80:80 nginx",
    ]
  }

  connection {
    type        = "ssh"
    user        = "root"
    private_key = file("~/.ssh/id_ed25519")
    host        = digitalocean_droplet.terraform-test-droplet.ipv4_address
  }
}

output "ip" {
  value = digitalocean_droplet.terraform-test-droplet.ipv4_address
}

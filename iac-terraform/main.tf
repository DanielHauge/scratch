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

resource "digitalocean_ssh_key" "mykey" {
  name       = "my-ssh-key"
  public_key = file("~/.ssh/id_ed25519.pub")
}

resource "digitalocean_droplet" "my-droplet_var_name" {
  name     = "my-cool-terraform-droplet" # The name of the droplet in DigitalOcean
  region   = "fra1"                      # Change to your preferred region
  size     = "s-1vcpu-1gb"               # Choose a plan (check DigitalOcean pricing)
  image    = "ubuntu-24-10-x64"          # Base image
  ssh_keys = [digitalocean_ssh_key.mykey.fingerprint]


  provisioner "remote-exec" {
    inline = [
      "sudo apt update -y",
      "sudo apt install -y nginx",
      "sudo systemctl enable --now nginx",
      "sudo apt-get update",
      "sudo apt-get install -y apt-transport-https ca-certificates curl software-properties-common",
      "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -",
      "sudo add-apt-repository 'deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable'",
      "sudo apt-get update",
      "sudo apt-get install -y docker-ce",
    ]
  }

  connection {
    type        = "ssh"
    user        = "root"
    private_key = file("~/.ssh/id_ed25519")
    host        = self.ipv4_address
  }
}



Nya is a free, open-source, portable and self-hostable PaaS. Deploy your apps to your own servers easily without worrying about cloud or vendor lock-in. Anything you can access via SSH can be a part of your platform. Production-ready features of Nya include:

- Automatic Kubernetes cluster setup
- Private Docker registry
- HTTPS and DNS Management
- Single command deployments
- A simple application framework

## Prerequisites 

**What you need:**
- One or more servers (VPS, bare metal, homelab - anything with Ubuntu 24.04)
- SSH access to your servers
- Docker installed locally (for building images)
- Rust/Cargo installed: https://rustup.rs

You'll also need to have the following tools installed on your machine:

- [Ansible](https://docs.ansible.com/projects/ansible/latest/installation_guide/intro_installation.html)
- [Docker](https://docs.docker.com/desktop/)

**Copy SSH keys to servers (one-time setup):**
```bash
ssh-add ~/.ssh/your_key
ssh-copy-id user@server-ip
```

Perform this for every server that you're planning on using in the platform. Once you've completed this once, you shouldn't have to do it again. 

### Concepts

There are a few terms that are referred to when using Nya: 

- **Base**: The application hosting apparatus of the platform. This is where Nya will run and host your applications. More technically, this is your Kubernetes cluster. 
- **Base Config**: The file responsible for holding the configuration of the base. 
- **Capsule**: A group of related applications that are managed and deployed together. For example: a monorepo. 
- **Pack**: A single deployable application. For example: a React SPA or Flask API. 

## Getting Started

Before using Nya, it's highly reccommended that your public SSH key is copied to your servers. Skipping this step can cause issues. 

**Install Nya:**
```bash
cargo install nya
```

**Configure Docker for insecure registry:**

On your local machine, add your control plane IP to Docker's insecure registries:

*Mac/Windows (Docker Desktop):*
Settings → Docker Engine → Add to JSON:
```json
{
  "insecure-registries": ["YOUR_CONTROL_PLANE_IP:5000"]
}
```

*Linux:*
```bash
sudo nano /etc/docker/daemon.json
# Add the same JSON config
sudo systemctl restart docker
```

### Create the Base:

First we need to create the Nya Base Config:

``` bash
nya init
```

This creates a new base config at `~/.nya/nya_base_config.json` by default. If you'd wish for the base config to be created in a specific location, you can add the `-o` or `--output` flag and provide a path.

```bash
Created Nya base config template
Location: /Users/_user_/.nya/nya_base_config.json

Next steps:
1. Edit the config file and fill in your infrastructure details
2. Run: nya base build
```
Update the base config that corresponds with your infrastructure. If you have more than one worker node, you can add another object: 
```json
      "hosts": {
        "worker1": {
          "ansible_host": "host",
          "ansible_user": "username",
          "ansible_ssh_private_key_file": "~/keyfile"
        },
        "worker22": {
          "ansible_host": "host",
          "ansible_user": "username",
          "ansible_ssh_private_key_file": "~/keyfile"
        }
        ...
      }
```

**Build the Base**: 

Once you've completed updating the base config, run 
```bash 
nya base build
``` 
This setups the platform. Provide the config with `-c` or `--config` if you provided a custom location. This should take a few minutes to complete. 
If you run into issues and wish to start over, or simply want to remove Nya from your servers, run 
```bash
nya base destroy
```

### Create Capsule

Once your base has been completed, navigate to your application. Create a new capsule by running
```bash
nya capsule new 
```

Give the capsule a name, and it will create the capsule's config. You can provide a custom location using `-c` or `--config`. 
```bash
> What do you want to call this capsule? test
Created new Capsule file at: /Users/_user_/.../test/.nya/nya.json
```

**Create Packs:**

You can now create packs that will be managed by the capsule. Run
```bash
nya pack new 
```
Give the pack a name and select what type of application the pack will be. If you selected a custom location for your capsule, you can provide it with `-c` or `--capsule``. You should now see a new directory created with the name of your pack. The directory should include a `Dockerfile` and a `values.yaml` file.
```bash
✓ Created pack: test-pack
Location: /Users/_user_/.../test/test-pack
Edit your Dockerfile, then run: nya pack deploy
```
> Note: Wherever you place your capsule, the expectation is that the capsule will be the parent directory of all of it's packs. If you decide to place the pack files in a different place, you'll have to manage the location yourself in the `nya.json` file. 

**Prepare Packs:**

Once you've created your pack, add everything you need for your application. Update the Dockerfile to builkd your container for your application. Make sure you've tested the container locally. 

### Ship It!

Once you've prepared your capsule and packs, and have verified that your containers are working as expected, run 
```bash
nya ship
```
to automatically deploy the capsule to the platform. If either your base config or your capsule are in custom locations, you can provide `-c` or `--config` for your base config's path, and `-l` or `--location` for your capsule's location. 

Your applications have now been successfully deployed to the Nya platform! 

### Accessing the Applications

The Nya base handles DNS for the platform as well, it's just a matter of directing requests to it. 

**For local access**

Navigate to your router or modem and update the Primary DNS to point to your control plane's IP address. 

Then, you should be able to access your applications from the browser.

Frontend: `https://{your pack's name}.{your_domain_name}`

Backend: `https://{your pack's name}-api.{your_domain_name}`

**To make apps accessible from anywhere**

**Option 1: Cloudflare Tunnel (Recommended)**
- No port forwarding needed
- Free TLS certificates

[Cloudflare Tunnel docs](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/)

**Option 2: Port Forwarding**
1. Forward ports 80/443 on your router → control plane IP
2. Point your domain's A record to your public IP
3. Apps accessible at `https://app.yourdomain.com`

## Known Limitations

- **Architecture**: All nodes must be same architecture (all x86_64 OR all ARM64)
  - There is currently no ARM support, but it is on the roadmap
- **OS**: Ubuntu 24.04 required on all nodes
- **Certificates**: Uses self-signed certs (warnings for public access without Cloudflare)

## Troubleshooting



## What's Next?

Nya is in active development. Planned for future releases:

**v0.2:**
- Replace Ansible with direct SSH
- Multi-OS support (Ubuntu + Debian)
- ARM support
- Local Nya observation tooling

**v0.3:**
- Better update reliability (SHA-based image tagging)
- Status/monitoring commands (`nya status`, `nya logs`)
- Single pack deployments (`nya pack deploy`)
- Nya library released (use Nya's event-driven paradigm in your own projects)

**v0.4:**
- Web dashboard
- Let's Encrypt integration (real TLS certificates)
- Multi-architecture support (mixed x86_64/ARM clusters)

**v0.5:**
- Plugin system  (extend Nya with custom functionality)

**Long Term Vision**
- Coomunity driven plugin ecosystem - allow all users and entities to provide their own Nya plugins and extend Nya's capabilities. 
- Decentralized infrastructure platform - empower creators with open-source alternatives to extractive platforms, fair pricing models, and tools that put control back in users' hands.


Want to contribute? Check out [CONTRIBUTING.md] open an issue!

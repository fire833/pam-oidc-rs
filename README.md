
## PAM-OIDC-RS

A custom PAM module to authenticate users via an OIDC provider, like Google, Github, Keycloak, etc. 

### Administration/Configuration

`pam-oidc-rs` is a PAM module, and thus requires appropriate configuration for the services which you wish to utilize the module in `/etc/pamd.d/*`. Here is a quide that does a better job than I can at explaining PAM modules and how to configure them: https://docs.rockylinux.org/guides/security/pam/. Basically, once you have built/downloaded the module, place it in `/lib/security/pam_oidc.so`, and then configure your services to use it. Please note however that there is a minor external configuration file that is required for giving `pam-oidc-rs` the knowledge it needs to successfully connect with your Authorization server/IdP.

The configuration file must be located at `/etc/pam_oidc/config.yaml`, and here are the keys that must exist within the file:

- `client_id` - The client ID for use with the IdP
- `client_secret` - The client secret for use with the IdP
- `issuer_url` - The top-level URL to access the IdP realm. This should be the root directory that you can access such that `$issuer_url/.well-known/openid-configuration` exists.

#### Example Configuration

```yaml
client_id: pamlocal
client_secret: verybadsecret
issuer_url: http://kc:8080/realms/demo
```

This is taken directly from the file in [./test/pam_config.yaml](./test/pam_config.yaml), feel free to copy/modify that configuration file as you see fit for your application.

### Developing

#### Prerequisites

- [Podman](https://docs.podman.io/en/latest/markdown/podman-compose.1.html) (or [Docker](https://docs.docker.com/compose/)) compose
- [Pulumi CLI](https://www.pulumi.com/docs/install/) (optional but highly recommended for deterministic configuration of the Keycloak instance)
- [Rust toolchain](https://www.rust-lang.org/learn/get-started)
- [Golang toolchain](https://go.dev/dl/)
- [pamtester](https://pamtester.sourceforge.net/) (used for testing the module)

In order to run/test the PAM module locally, the recommended method is using docker compose (or `podman-compose`, which I will use in this case but feel free to switch out `podman-compose` for `docker compose` if you want). You can begin the stack with the following command

```bash
# When running in the root directory of the repo

podman-compose up
```

Then, in another terminal, you can provision the keycloak instance running locally with the following:

```bash
# When running in the root directory of the repo

./scripts/stackup
```

Feel free to inspect this 

You can now exec into the `pam_svc` container in order to test logging in

```bash
podman-compose exec pam_svc bash

# You can then run pamtester in order to validate your module 
# loads and authenticates john@t.co, his password should just 
# be "pass".
[root@a2a7390bd3b4 /] pamtester test john@t.co authenticate
Password: #insert "pass" here
pamtester: successfully authenticated @ successful authentication attempt

# Now, you will be able to look at the event logs in Keycloak 
# and indeed see that john@t.co logged in from your container's 
# IP address!
```

The Keycloak instance that gets spun up runs on port 8000, so you can access it in your browser at `http://localhost:8000/`, and the credentials to enter will be `admin` as the username and `pass` for the password. You will also want to look at the demo relam by selecting the appropriate realm in the top left corner of the admin panel.

# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"

  config.vm.provision "shell", inline: <<-SHELL
    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --yes
  SHELL
end

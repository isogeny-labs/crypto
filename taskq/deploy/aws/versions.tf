terraform {
  required_version = ">= 1.6"
  required_providers {
    aws        = { source = "hashicorp/aws", version = ">= 6.59, < 7.0" }
    kubernetes = { source = "hashicorp/kubernetes", version = "~> 2.32" }
    helm       = { source = "hashicorp/helm", version = "~> 2.15" }
    random     = { source = "hashicorp/random", version = "~> 3.6" }
  }

  # State holds the ElastiCache AUTH token: keep it in an encrypted remote
  # backend, never in git. Copy backend.hcl.example to backend.hcl, fill it in,
  # and run `terraform init -backend-config=backend.hcl`.
  backend "s3" {}
}

provider "aws" {
  region = var.region
  default_tags {
    tags = merge({ project = "taskq", managed-by = "terraform" }, var.tags)
  }
}

provider "aws" {
  alias  = "us_east_1"
  region = "us-east-1"
}

# Kubernetes and Helm talk to the cluster this configuration creates, with a
# short-lived token from the caller's AWS credentials.
provider "kubernetes" {
  host                   = module.eks.cluster_endpoint
  cluster_ca_certificate = base64decode(module.eks.cluster_certificate_authority_data)
  exec {
    api_version = "client.authentication.k8s.io/v1beta1"
    command     = "aws"
    args        = ["eks", "get-token", "--cluster-name", module.eks.cluster_name, "--region", var.region]
  }
}

provider "helm" {
  kubernetes {
    host                   = module.eks.cluster_endpoint
    cluster_ca_certificate = base64decode(module.eks.cluster_certificate_authority_data)
    exec {
      api_version = "client.authentication.k8s.io/v1beta1"
      command     = "aws"
      args        = ["eks", "get-token", "--cluster-name", module.eks.cluster_name, "--region", var.region]
    }
  }
}

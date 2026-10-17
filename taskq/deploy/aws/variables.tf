variable "region" {
  description = "AWS region. us-west-2 matches ecc2k130/aws."
  type        = string
  default     = "us-west-2"
}

variable "name" {
  description = "Prefix for every resource (cluster, cache, file system)."
  type        = string
  default     = "taskq"
}

variable "tags" {
  description = "Extra tags on every resource."
  type        = map(string)
  default     = {}
}

variable "vpc_cidr" {
  type    = string
  default = "10.42.0.0/16"
}

variable "cluster_version" {
  type    = string
  default = "1.36"
}

variable "cluster_endpoint_public_access_cidrs" {
  description = "Who may reach the Kubernetes API. Terraform itself needs it; narrow it to your own addresses."
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

# ---- Karpenter Spot worker pools -------------------------------------------

variable "cpu_spot_instance_types" {
  description = "Multiple eligible shapes improve Spot availability; every one is Spot."
  type        = list(string)
  default     = ["c7i.2xlarge", "c7a.2xlarge", "c6i.2xlarge", "m7i.2xlarge"]
  validation {
    condition     = length(var.cpu_spot_instance_types) > 0
    error_message = "At least one CPU Spot instance type is required."
  }
}

variable "gpu_spot_instance_types" {
  description = "GPU Spot shapes. Empty disables the GPU NodePool and device plugin."
  type        = list(string)
  default     = []
}

variable "karpenter_ami_alias" {
  description = "Pin an EKS AL2023 AMI release before production use, for example al2023@vYYYYMMDD."
  type        = string
  default     = "al2023@latest"
}

# ---- Redis ------------------------------------------------------------------

variable "redis_node_type" {
  type    = string
  default = "cache.r7g.large"
}

variable "redis_engine_version" {
  description = "Redis OSS 7.x: taskq needs XAUTOCLAIM (>= 6.2) and Lua."
  type        = string
  default     = "7.1"
}

# ---- workloads --------------------------------------------------------------

variable "image" {
  description = "Default worker image. Pin a sha tag so every result's TASKQ_IMAGE names what ran."
  type        = object({ repository = string, tag = string })
  default     = { repository = "ghcr.io/aburan28/taskq-worker", tag = "latest" }
}

variable "repos" {
  description = "Worker repo allowlist: alias -> clone URL."
  type        = map(string)
  default = {
    crypto                = "https://github.com/aburan28/crypto.git"
    crypto-autoresearcher = "https://github.com/aburan28/crypto-autoresearcher.git"
    cryptanalysis         = "https://github.com/aburan28/cryptanalysis.git"
  }
}

variable "pools" {
  description = <<-EOT
    Worker pools, one Deployment each. node_group is "cpu" or "gpu" and decides
    the node selector and tolerations. cpu/memory are both request and limit
    (Guaranteed QoS, so the static CPU manager gives the pod exclusive cores).
    image overrides var.image (e.g. the cryptanalysis toolchain images).
  EOT
  type = list(object({
    name       = string
    queues     = list(string)
    replicas   = number
    node_group = string
    cpu        = string
    memory     = string
    gpus       = optional(number, 0)
    labels     = optional(map(string), {})
    image      = optional(object({ repository = string, tag = string }))
    # KEDA scaler for this pool's queues; only used when enable_keda = true.
    keda = optional(object({
      min_replicas = optional(number, 0)
      max_replicas = optional(number, 10)
      lag_count    = optional(number, 1)
    }))
  }))
  default = [
    { name = "cpu", queues = ["cpu"], replicas = 0, node_group = "cpu",
      cpu  = "6", memory = "12Gi", labels = { pool = "cpu-spot" },
    keda = { min_replicas = 0, max_replicas = 20, lag_count = 1 } },
  ]
  validation {
    condition     = alltrue([for p in var.pools : contains(["cpu", "gpu"], p.node_group)])
    error_message = "pools[*].node_group must be \"cpu\" or \"gpu\"."
  }
}

variable "namespace" {
  type    = string
  default = "taskq"
}

variable "enable_keda" {
  description = "KEDA scales worker pods from zero; Karpenter scales Spot nodes for Pending pods."
  type        = bool
  default     = true
}

variable "results_storage" {
  description = "Nominal PVC size; EFS is elastic and bills for what is stored."
  type        = string
  default     = "100Gi"
}

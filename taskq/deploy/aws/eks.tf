module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 21.26"

  name               = var.name
  kubernetes_version = var.cluster_version

  endpoint_public_access                   = true
  endpoint_public_access_cidrs             = var.cluster_endpoint_public_access_cidrs
  enable_cluster_creator_admin_permissions = true

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  addons = {
    coredns                = {}
    kube-proxy             = {}
    vpc-cni                = {}
    eks-pod-identity-agent = {}
    aws-efs-csi-driver = {
      service_account_role_arn = module.efs_csi_irsa.iam_role_arn
    }
  }

  # Karpenter must run on nodes it does not manage. This group is Spot too:
  # capacity shortages leave pods Pending, never launch On-Demand nodes.
  eks_managed_node_groups = {
    system = {
      ami_type       = "AL2023_x86_64_STANDARD"
      instance_types = ["m6i.large", "m6a.large", "m5.large"]
      capacity_type  = "SPOT"
      min_size       = 1
      desired_size   = 2
      max_size       = 3
      labels         = { "karpenter.sh/controller" = "true" }
    }
  }

  node_security_group_tags = { "karpenter.sh/discovery" = var.name }
}

module "efs_csi_irsa" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts-eks"
  version = "~> 5.44"

  role_name_prefix      = "${var.name}-efs-csi-"
  attach_efs_csi_policy = true
  oidc_providers = {
    main = {
      provider_arn               = module.eks.oidc_provider_arn
      namespace_service_accounts = ["kube-system:efs-csi-controller-sa"]
    }
  }
}

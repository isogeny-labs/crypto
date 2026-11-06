# The EKS module's Karpenter submodule owns IAM, Pod Identity, the SQS
# interruption queue, and EventBridge routing. The controller runs on the
# independent Spot system group in eks.tf.
module "karpenter" {
  source  = "terraform-aws-modules/eks/aws//modules/karpenter"
  version = "~> 21.26"

  cluster_name                    = module.eks.cluster_name
  create_pod_identity_association = true
  node_iam_role_use_name_prefix   = false
  node_iam_role_name              = "${var.name}-karpenter-node"
}

data "aws_ecrpublic_authorization_token" "karpenter" {
  provider = aws.us_east_1
}

resource "helm_release" "karpenter" {
  name                = "karpenter"
  namespace           = "kube-system"
  repository          = "oci://public.ecr.aws/karpenter"
  repository_username = data.aws_ecrpublic_authorization_token.karpenter.user_name
  repository_password = data.aws_ecrpublic_authorization_token.karpenter.password
  chart               = "karpenter"
  version             = "1.14.1"
  wait                = true

  values = [yamlencode({
    serviceAccount = { name = module.karpenter.service_account }
    nodeSelector   = { "karpenter.sh/controller" = "true" }
    settings = {
      clusterName       = module.eks.cluster_name
      clusterEndpoint   = module.eks.cluster_endpoint
      interruptionQueue = module.karpenter.queue_name
    }
  })]

  depends_on = [module.eks, module.karpenter]
}

# Helm creates the CRDs with the controller before applying these custom
# resources. Both NodePools explicitly allow Spot only.
resource "helm_release" "spot_nodepools" {
  name      = "taskq-spot-nodepools"
  chart     = "${path.module}/../helm/spot-nodepools"
  namespace = "kube-system"
  wait      = true
  values = [yamlencode({
    clusterName      = module.eks.cluster_name
    nodeRole         = module.karpenter.node_iam_role_name
    amiAlias         = var.karpenter_ami_alias
    cpuInstanceTypes = var.cpu_spot_instance_types
    gpuInstanceTypes = var.gpu_spot_instance_types
  })]
  depends_on = [helm_release.karpenter]
}

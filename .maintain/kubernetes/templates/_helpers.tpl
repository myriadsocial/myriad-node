{{/*
Expand the name of the chart.
*/}}
{{- define "myriad-node.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "myriad-node.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "myriad-node.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "myriad-node.labels" -}}
helm.sh/chart: {{ include "myriad-node.chart" . }}
{{ include "myriad-node.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "myriad-node.selectorLabels" -}}
app.kubernetes.io/name: {{ include "myriad-node.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}


{{/*
Create the name of the service account to use
*/}}
{{- define "myriad-node.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "myriad-node.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Custom chain-spec path
*/}}
{{- define "myriad-node.customChainSpecPath" -}}
{{- printf "%s/%s" .Values.image.basePath "chain-spec" }}
{{- end }}

{{/*
Relay custom chain-spec
*/}}
{{- define "myriad-node.relayCustomChainSpec" -}}
{{- printf "%s/%s" (include "myriad-node.relayCustomChainSpecPath" .) .Values.node.relayChainFileName }}
{{- end }}

{{/*
Relay chain-spec path
*/}}
{{- define "myriad-node.relayCustomChainSpecPath" -}}
{{- printf "%s/%s" .Values.image.basePath "relay-chain-spec" }}
{{- end }}

{{/*
Custom chain-spec
*/}}
{{- define "myriad-node.customChainSpec" -}}
{{- printf "%s/%s" (include "myriad-node.customChainSpecPath" .) .Values.node.chainFileName }}
{{- end }}

{{/*
Node key Secret
*/}}
{{- define "myriad-node.nodeKeySecret" -}}
{{- printf "%s-%s" "node-key" (include "myriad-node.fullname" .) }}
{{- end }}

{{/*
Node key
*/}}
{{- define "myriad-node.nodeKey" -}}
{{- printf "%s/%s" .Values.image.basePath "node-key" }}
{{- end }}

{{/*
Session key secret
*/}}
{{- define "myriad-node.sessionKeySecret" -}}
{{- printf "%s-%s" "session-key" (include "myriad-node.fullname" .) }}
{{- end }}

{{/*
P2P service
*/}}
{{- define "myriad-node.p2pService" -}}
{{- printf "%s-%s" "p2p" (include "myriad-node.fullname" .) }}
{{- end }}

{{/*
HTTP RPC service
*/}}
{{- define "myriad-node.httpRpcService" -}}
{{- printf "%s-%s" "http-rpc" (include "myriad-node.fullname" .) }}
{{- end }}

{{/*
P2P ingress
*/}}
{{- define "myriad-node.p2pIngress" -}}
{{- printf "%s-%s" "p2p" (include "myriad-node.fullname" .) }}
{{- end }}

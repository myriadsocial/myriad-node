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
Create the name of the service http-rpc
*/}}
{{- define "myriad-node.serviceHttpRPCName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "http-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the service websocket-rpc
*/}}
{{- define "myriad-node.serviceWebsocketRPCName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "websocket-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the ingress http-rpc
*/}}
{{- define "myriad-node.ingressHttpRPCName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "http-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the ingress websocket-rpc
*/}}
{{- define "myriad-node.ingressWebsocketRPCName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "websocket-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of node key secret.
*/}}
{{- define "myriad-node.nodeKey" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "node-key" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/* Returns the default name to use with session injection related resources */}}
{{- define "myriad-node.sessionInjectionJobName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "session-injection-job" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of session key secret.
*/}}
{{- define "myriad-node.sessionKey" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "session-key" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
{{/* Returns the default name to use with pod restarter  related resources */}}
{{- define "myriad-node.podRestarterCronJobName" -}}
{{- printf "%s-%s" (include "myriad-node.fullname" .) "pod-restarter-cron-job" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}
*/}}
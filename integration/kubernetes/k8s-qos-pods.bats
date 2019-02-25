#!/usr/bin/env bats
#
# Copyright (c) 2018 Intel Corporation
#
# SPDX-License-Identifier: Apache-2.0
#

load "${BATS_TEST_DIRNAME}/../../.ci/lib.sh"
TEST_INITRD="${TEST_INITRD:-no}"
issue="https://github.com/kata-containers/runtime/issues/1127"
memory_issue="https://github.com/kata-containers/runtime/issues/1249"

setup() {
	skip "test not working see: ${issue}, ${memory_issue}"
	export KUBECONFIG=/etc/kubernetes/admin.conf

	if sudo -E kubectl get runtimeclass | grep kata; then
		pod_config_dir="${BATS_TEST_DIRNAME}/runtimeclass_workloads"
	else
		pod_config_dir="${BATS_TEST_DIRNAME}/untrusted_workloads"
	fi
}

@test "Guaranteed QoS" {
	skip "test not working see: ${issue}, ${memory_issue}"

	pod_name="qos-test"

	# Create pod
	sudo -E kubectl create -f "${pod_config_dir}/pod-guaranteed.yaml"

	# Check pod creation
	sudo -E kubectl wait --for=condition=Ready pod "$pod_name"

	# Check pod class
	sudo -E kubectl get pod "$pod_name" --output=yaml | grep "qosClass: Guaranteed"
}

@test "Burstable QoS" {
	skip "test not working see: ${issue}, ${memory_issue}"

	pod_name="burstable-test"

	# Create pod
	sudo -E kubectl create -f "${pod_config_dir}/pod-burstable.yam"l

	# Check pod creation
	sudo -E kubectl wait --for=condition=Ready pod "$pod_name"

	# Check pod class
	sudo -E kubectl get pod "$pod_name" --output=yaml | grep "qosClass: Burstable"
}

@test "BestEffort QoS" {
	skip "test not working see: ${issue}, ${memory_issue}"
	pod_name="besteffort-test"

	# Create pod
	sudo -E kubectl create -f "${pod_config_dir}/pod-besteffort.yam"l

	# Check pod creation
	sudo -E kubectl wait --for=condition=Ready pod "$pod_name"

	# Check pod class
	sudo -E kubectl get pod "$pod_name" --output=yaml | grep "qosClass: BestEffort"
}

teardown() {
	skip "test not working see: ${issue}, ${memory_issue}"
	kubectl delete pod "$pod_name"
}

// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

use grpc_cloud_adapter::grpc_cloud_adapter::GRPCCloudAdapter;
use ibeji_adapter::ibeji_adapter::IbejiAdapter;
use in_memory_mock_mapping_adapter::in_memory_mock_mapping_adapter::InMemoryMockMappingAdapter;
use managed_subscribe_data_adapter::managed_subscribe_data_adapter_factory::ManagedSubscribeDataAdapterFactory;
use mqtt_data_adapter::mqtt_data_adapter_factory::MqttDataAdapterFactory;
use sample_grpc_data_adapter::sample_grpc_data_adapter_factory::SampleGRPCDataAdapterFactory;

freyja::freyja_main! {
    IbejiAdapter,
    GRPCCloudAdapter,
    InMemoryMockMappingAdapter,
    [SampleGRPCDataAdapterFactoryGRPCDataAdapterFactory, ManagedSubscribeDataAdapterFactory, MqttDataAdapterFactory],
}

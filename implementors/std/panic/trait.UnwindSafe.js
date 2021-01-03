(function() {var implementors = {};
implementors["avmath"] = [{"text":"impl !UnwindSafe for Layer","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for GeometricAltitude","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for GeopotentialAltitude","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for PressureAltitude","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for DensityAltitude","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for AltimeterSetting","synthetic":true,"types":[]}];
implementors["gauge_sys"] = [{"text":"impl UnwindSafe for FsContext","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawServiceId","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GaugeDrawData","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawUnit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawAircraftVariable","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawNamedVariable","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ServiceId","synthetic":true,"types":[]}];
implementors["simconnect_sys"] = [{"text":"impl UnwindSafe for SimConnect","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataDefinition","synthetic":true,"types":[]},{"text":"impl&lt;Group&gt; UnwindSafe for NotificationGroupDefinition&lt;Group&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Group: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;EventType&gt; UnwindSafe for EventDefinition&lt;EventType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;EventType: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ReceiveHeader","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ReceiveOpen","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Version","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ReceiveEvent","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawDataDefinitionId","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawObjectId","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawNotificationGroupId","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawEventId","synthetic":true,"types":[]},{"text":"impl UnwindSafe for NotificationGroupPriority","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawDataSetFlag","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawMessageType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RawDataType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SimConnectHandle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Handle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for WindowHandle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for HResult","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataSetFlag","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MessageType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DataType","synthetic":true,"types":[]}];
implementors["wt_cj4"] = [{"text":"impl !UnwindSafe for FadecController","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ThrottleAxis","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ThrustValue","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ThrottlePercent","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ThrottleMode","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for EngineData&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EngineNumber","synthetic":true,"types":[]}];
implementors["wt_systems"] = [{"text":"impl&lt;In&gt; !UnwindSafe for PidConfiguration&lt;In&gt;","synthetic":true,"types":[]},{"text":"impl&lt;In&gt; !UnwindSafe for PidController&lt;In&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
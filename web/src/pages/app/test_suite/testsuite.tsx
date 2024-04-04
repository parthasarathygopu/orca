import { PageHeader } from "core/components/page_header";
import { useEffect, useState } from "react";
import { PlusIcon, PlayCircleIcon, TrashIcon } from "@heroicons/react/24/outline";
import { useParams } from "react-router-dom";
import {
  Button, Flex, IconButton,
  Tooltip
} from "@radix-ui/themes";
import { ColumnField } from "core/components/table";
import { ReadOnlyTable } from "core/components/table/read";
import { SearchableDropdown } from "core/components/dropdown/index.jsx";
import { fetchTestCases } from "service/app/test_case";
import { fetchSuiteItems, batchUpdate } from "service/app/test_suite";
import "./index.css";

export function TestSuitePage() {
  const { appId = "", testSuiteId = "" } = useParams();
  const [testCaseList, setTestCaseList] = useState([] as any);
  const [testSuiteDetails, setTestSuiteDetails] = useState([] as any);
  const [testCases, setTestCases] = useState([] as any);
  const [selectedTestCase, setSelectedTestCase] = useState({} as any);


  const getCaseList = () => {
    fetchTestCases(appId).then((cases: any) => {
      setTestCaseList(cases);
    })
      .finally(() => { });
  }

  const getTestSuiteDetails = () => {
    fetchSuiteItems(appId, testSuiteId).then((suites: any) => {
      setTestSuiteDetails(suites);
      setTestCases(suites.suite_execution)
    })
      .finally(() => { });
  }

  useEffect(() => {
    getTestSuiteDetails();
    getCaseList()
  }, [appId]);

  const handleRun = () => {
    // Service.post(`${Endpoint.v1.suite.run(appId, testSuiteId)}`).finally(() =>
    //   setIsRunning(false)
    // );
  };


  const columns: Array<ColumnField> = [
    {
      key: "name",
      label: "Test case name",
      className: "flex-auto "
    },
    {
      key: "description",
      label: "Description",
      className: "flex-auto "
    },
    {
      key: "action",
      label: "Action",
      className: "flex-initial w-48",
      render: (_: string, record: any) => {
        return (
          <Flex align="center" gap="3">
            <Tooltip content="Delete">
              <IconButton
                className="cursor-pointer"
                color="red"
                variant="soft"
                onClick={() => {
                  const newList = testCases.filter((data: any) => data.id !== record.id);
                  updateTestCase(newList);
                }}
              >
                <TrashIcon className="size-4" />
              </IconButton>
            </Tooltip>
          </Flex>
        );
      }
    }
  ];

  const updateTestCase = (updatedList: any) => {
    setSelectedTestCase({});
    const newList: [] = updatedList.map((data: any, index: number) => {
      return { reference: data.id,  execution_order: index + 1, type_field: "TestCase", suite_id: testSuiteId };
    });
    setTestCases(newList);

    batchUpdate(appId, testSuiteId, newList)
      .then((cases: any) => { })
      .finally(() => { });
  };

  return (
    <>
      <PageHeader
        backIcon
        title={testSuiteDetails.name}
        extra={
          <div className=" flex items-center gap-3">
            <Button
              variant="soft"
              className="flex items-center gap-3"
              onClick={handleRun}
              color="indigo"
            >
              <PlayCircleIcon className="size-4" /> Dry Run
            </Button>
          </div>
        }
      />
      <div>
        <div className="selectTestCase">
          <h1><b>Select test case</b></h1>
          <SearchableDropdown
            options={testCaseList || []}
            label="name"
            id="id"
            selectedValue={selectedTestCase}
            handleChange={(val: any) => {
              setSelectedTestCase(val);
            }}
          />
          <Button
            variant="soft"
            onClick={() => {
              selectedTestCase.id && updateTestCase([...testCases, selectedTestCase]);
            }}
          >
            <PlusIcon width="16" height="16" />
            Add
          </Button>
        </div>
        <ReadOnlyTable
          column={columns}
          data={testCases}
          showPagination={false}
          onDragEnd={(data: any) => updateTestCase(data)}
          isDragAllowed={true}
        />
      </div>
    </>
  );
}
